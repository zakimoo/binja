use darling::FromField;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::{
    attribute::{FieldAttributes, StructAttributes},
    bit_field::{flush_bit_field_at_end, flush_bit_field_if_needed, gen_bit_field_serialization},
};

pub fn generate_struct_binary_serialize(
    data: &syn::DataStruct,
    attributes: &StructAttributes,
) -> TokenStream {
    let name = &attributes.ident;
    let generics = &attributes.generics;
    let mut gen_clone = generics.clone();

    // Add trait bounds to each type parameter
    let where_clause = gen_clone.make_where_clause();
    for param in generics.type_params() {
        let ident = &param.ident;
        where_clause.predicates.push(parse_quote! {
            #ident: ::binja::serializer::BinarySerialize
        });
    }

    let fields_token = match &data.fields {
        // struct Example { field: String }
        syn::Fields::Named(fields_named) => {
            let serialize_fields = gen_ser_fields(&fields_named.named, |f, _| {
                let ident = &f.ident;
                // struct Example { field: String }
                // ::binja::serializer::binary_serialize(&self.field, serializer)?;
                return quote! { &self.#ident };
            });

            quote! {
                #(#serialize_fields)*
            }
        }
        // struct Example(String) , struct Example(String, String)
        syn::Fields::Unnamed(fields_unnamed) => {
            let serialize_fields = gen_ser_fields(&fields_unnamed.unnamed, |_, i| {
                let index = syn::Index::from(i);
                // struct Example(u8)
                // ::binja::serializer::binary_serialize(&self.0, serializer)?;
                quote! {
                    &self.#index
                }
            });
            quote! {
                #(#serialize_fields)*
            }
        }
        // struct Example;
        syn::Fields::Unit => {
            quote! {}
        }
    };

    TokenStream::from(quote! {
        impl #generics binja::serializer::BinarySerialize for #name #generics #where_clause{
            fn binary_serialize(&self, serializer: &mut ::binja::serializer::BinarySerializer) -> binja::error::Result<()> {
                #fields_token
                Ok(())
            }
        }
    })
}

pub fn generate_struct_binary_parse(
    data: &syn::DataStruct,
    attributes: &StructAttributes,
) -> TokenStream {
    let name = &attributes.ident;
    let generics = &attributes.generics;
    let mut gen_clone = generics.clone();

    // Add trait bounds to each type parameter
    let where_clause = gen_clone.make_where_clause();
    for param in generics.type_params() {
        let ident = &param.ident;
        where_clause.predicates.push(parse_quote! {
            #ident: ::binja::parser::BinaryParse
        });
    }

    let fields_token = match &data.fields {
        // struct Example { field: String }
        syn::Fields::Named(fields_named) => {
            let parse_fields = gen_par_named_fields(fields_named, true);

            quote! {
                #(#parse_fields)*
            }
        }

        // struct Example(String) , struct Example(String, String)
        syn::Fields::Unnamed(fields_unnamed) => {
            let parse_fields = gen_par_unnamed_fields(fields_unnamed, true);
            // 0: binja::parser::binary_parse(parser)?,
            // 1: binja::parser::binary_parse(parser)?,
            // 2: binja::parser::binary_parse(parser)?,
            quote! {
                #(#parse_fields)*
            }
        }

        // struct Example;
        syn::Fields::Unit => {
            quote! {}
        }
    };

    TokenStream::from(quote! {
        impl #generics binja::parser::BinaryParse for #name #generics #where_clause{
            fn binary_parse(parser: &mut ::binja::parser::BinaryParser) -> binja::error::Result<Self> {
                // Ok(Self{
                // field: binja::parser::binary_parse(parser)?,
                // })
                Ok(Self{
                    #fields_token
                })
            }
        }
    })
}

pub fn gen_ser_fields<F>(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    mut get_field_expr: F,
) -> Vec<proc_macro2::TokenStream>
where
    F: FnMut(&syn::Field, usize) -> proc_macro2::TokenStream,
{
    let mut code = Vec::new();
    let mut bit_field_declared = false;
    let mut bit_offset = 0u8;

    for (i, f) in fields.iter().enumerate() {
        // Check if the field has attributes
        let attrs = FieldAttributes::from_field(f).unwrap_or_default();

        // skip field
        if attrs.skip() {
            continue;
        }

        let field_expr = get_field_expr(f, i);

        // if field have #[binja(bits = 6)]
        if let Some(bits) = attrs.bits() {
            // only construct bit field if it is not already constructed
            // if there is a field with bits
            if !bit_field_declared {
                code.push(quote! { let mut bit_field = 0u8; });
                bit_field_declared = true;
            }

            let allow_overflow = !attrs.no_overflow();

            gen_bit_field_serialization(
                &mut code,
                &field_expr,
                bits,
                &mut bit_offset,
                allow_overflow,
            );
        } else {
            // if last field is a bit field smaller that 8 bits
            // current field is not a bit field
            // flush last bit field
            flush_bit_field_if_needed(&mut code, &mut bit_offset);

            // serialize the current field
            code.push(quote! {
                ::binja::serializer::binary_serialize(#field_expr, serializer)?;
            })
        }
    }

    // if last field is a bit field smaller that 8 bits
    flush_bit_field_at_end(&mut code, bit_offset);

    code
}

pub fn gen_par_named_fields(
    fields: &syn::FieldsNamed,
    _is_struct: bool,
) -> Vec<proc_macro2::TokenStream> {
    let mut code = Vec::new();
    for f in fields.named.iter() {
        // Check if the field has attributes
        let attrs = FieldAttributes::from_field(f).unwrap_or_default();
        let ident = &f.ident;

        // #[derive(BinaryParse)]
        // struct Example { #[binja(skip)]  field: String ,field2: u8 }
        // Ok(Self{
        //     field: Default::default(), // skipped fields are defaulted for parsing
        //     field2: binja::parser::binary_parse(parser)?,
        // })
        if attrs.skip() {
            // skipped fields are defaulted
            code.push(quote! {
                #ident: Default::default(),
            });

            continue;
        }

        code.push(quote! {
            #ident: ::binja::parser::binary_parse(parser)?,
        });
    }

    code
}

pub fn gen_par_unnamed_fields(
    fields: &syn::FieldsUnnamed,
    _is_struct: bool,
) -> Vec<proc_macro2::TokenStream> {
    let mut code = Vec::new();

    for (i, f) in fields.unnamed.iter().enumerate() {
        // Check if the field has attributes
        let attrs = FieldAttributes::from_field(f).unwrap_or_default();
        let index = syn::Index::from(i);
        // #[derive(BinaryParse)]
        // struct Example(#[binja(skip)] u8, u32)
        // Ok(Self(
        //    0: Default::default(), // skipped fields are defaulted for parsing
        //     1: ::binja::parser::binary_parse(parser)?,
        // ))
        if attrs.skip() {
            code.push(quote! {
                #index: Default::default(),
            })
        } else {
            code.push(quote! {
                #index: ::binja::parser::binary_parse(parser)?,
            })
        }
    }
    code
}
