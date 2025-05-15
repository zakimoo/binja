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
                quote! { &self.#ident }
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

    let code = match &data.fields {
        // struct Example { field: String }
        syn::Fields::Named(fields_named) => gen_par_fields(
            &fields_named.named,
            |f, _| {
                let ident = &f.ident;
                // struct Example { field: String }
                // Ok(Self{
                //     field: Default::default(), // skipped fields are defaulted for parsing
                //     field2: binja::parser::binary_parse(parser)?,
                // })
                quote! {
                    #ident
                }
            },
            |fields| {
                // struct Example { field: String }
                // Ok(Self{
                //     field: Default::default(), // skipped fields are defaulted for parsing
                //     field2: binja::parser::binary_parse(parser)?,
                // })
                quote! {
                    Ok(Self {
                        #fields
                    })
                }
            },
        ),

        // struct Example(String) , struct Example(String, String)
        syn::Fields::Unnamed(fields_unnamed) => gen_par_fields(
            &fields_unnamed.unnamed,
            |_, i| {
                let ident = syn::Ident::new(&format!("field_{i}"), proc_macro2::Span::call_site());
                // struct Example { field: String }
                // Ok(Self{
                //     field: Default::default(), // skipped fields are defaulted for parsing
                //     field2: binja::parser::binary_parse(parser)?,
                // })
                quote! {
                    #ident
                }
            },
            |fields| {
                // struct Example { field: String }
                // Ok(Self{
                //     field: Default::default(), // skipped fields are defaulted for parsing
                //     field2: binja::parser::binary_parse(parser)?,
                // })
                quote! {
                    Ok(Self (
                        #fields
                    ))
                }
            },
        ),

        // struct Example;
        syn::Fields::Unit => quote! {Ok(Self {})},
    };

    TokenStream::from(quote! {
        impl #generics binja::parser::BinaryParse for #name #generics #where_clause{
            fn binary_parse(parser: &mut ::binja::parser::BinaryParser) -> binja::error::Result<Self> {
               #code
            }
        }
    })
}

pub fn gen_ser_fields<F>(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    get_field_expr: F,
) -> Vec<proc_macro2::TokenStream>
where
    F: Fn(&syn::Field, usize) -> proc_macro2::TokenStream,
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

pub fn gen_par_fields<F1, F2>(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    get_field_expr: F1,
    get_return_code: F2,
) -> proc_macro2::TokenStream
where
    F1: Fn(&syn::Field, usize) -> proc_macro2::TokenStream,
    F2: Fn(&proc_macro2::TokenStream) -> proc_macro2::TokenStream,
{
    let mut code = Vec::new();
    let mut fields_code = Vec::new();

    for (i, f) in fields.iter().enumerate() {
        // Check if the field has attributes
        let attrs = FieldAttributes::from_field(f).unwrap_or_default();
        let ident = get_field_expr(f, i);

        fields_code.push(ident.clone());

        // #[derive(BinaryParse)]
        // struct Example { #[binja(skip)]  field: String ,field2: u8 }
        // Ok(Self{
        //     field: Default::default(), // skipped fields are defaulted for parsing
        //     field2: binja::parser::binary_parse(parser)?,
        // })
        if attrs.skip() {
            // skipped fields are defaulted
            code.push(quote! {
              let  #ident= Default::default();
            });

            continue;
        }

        code.push(quote! {
           let #ident= ::binja::parser::binary_parse(parser)?;
        });
    }

    let return_code = get_return_code(&quote! {
       #(#fields_code),*
    });

    quote! {
        #(#code)*
        #return_code
    }
}
