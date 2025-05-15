use darling::FromField;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::attribute::{FieldAttributes, StructAttributes};

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
            let serialize_fields = generate_serialize_named_fields(fields_named, true);
            quote! {
                #(#serialize_fields)*
            }
        }
        // struct Example(String) , struct Example(String, String)
        syn::Fields::Unnamed(fields_unnamed) => {
            let serialize_fields = generate_serialize_unnamed_fields(fields_unnamed, true);
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
            let parse_fields = generate_parse_named_fields(fields_named, true);

            quote! {
                #(#parse_fields)*
            }
        }

        // struct Example(String) , struct Example(String, String)
        syn::Fields::Unnamed(fields_unnamed) => {
            let parse_fields = generate_parse_unnamed_fields(fields_unnamed, true);
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

pub fn generate_serialize_named_fields(
    fields: &syn::FieldsNamed,
    is_struct: bool,
) -> Vec<proc_macro2::TokenStream> {
    let mut code = Vec::new();

    for f in fields.named.iter() {
        // Check if the field has attributes
        let attrs = FieldAttributes::from_field(f).unwrap_or_default();
        let ident = &f.ident;

        // skip field
        if attrs.skip() {
            continue;
        }

        let field_expr = if is_struct {
            // struct Example { field: String }
            // ::binja::serializer::binary_serialize(&self.field, serializer)?;
            quote! {
                &self.#ident
            }
        } else {
            // enum Example { A { field: String } }
            // match self{
            //     Self::A { field } => {
            //         ::binja::serializer::binary_serialize(field, serializer)?,
            //     }
            quote! {
                #ident
            }
        };

        // if let Some(_bits) = opts.bits() {
        //     None
        // } else {
        code.push(quote! {
            ::binja::serializer::binary_serialize(#field_expr, serializer)?;
        })
        // }
    }

    code
}

pub fn generate_parse_named_fields(
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
            })
        } else {
            code.push(quote! {
                #ident: ::binja::parser::binary_parse(parser)?,
            })
        }
    }

    code
}

pub fn generate_serialize_unnamed_fields(
    fields: &syn::FieldsUnnamed,
    is_struct: bool,
) -> Vec<proc_macro2::TokenStream> {
    let mut code = Vec::new();

    for (i, f) in fields.unnamed.iter().enumerate() {
        // Check if the field has attributes
        let attrs = FieldAttributes::from_field(f).unwrap_or_default();
        let index = syn::Index::from(i);

        // skip field
        if attrs.skip() {
            continue;
        }

        let field_expr = if is_struct {
            // struct Example(u8)
            // ::binja::serializer::binary_serialize(&self.0, serializer)?;
            quote! {
                &self.#index
            }
        } else {
            // enum Example { A { field: String } }
            // match self{
            //     Self::A { field_#index } => {
            //         ::binja::serializer::binary_serialize(field_#index, serializer)?,
            //     }
            let ident = syn::Ident::new(&format!("field_{i}"), proc_macro2::Span::call_site());
            quote! {
                #ident
            }
        };

        code.push(quote! {
            ::binja::serializer::binary_serialize(#field_expr, serializer)?;
        })
    }
    code
}

pub fn generate_parse_unnamed_fields(
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
