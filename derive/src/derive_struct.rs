use darling::FromField;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::attribute::{FieldAttributes, StructAttributes};

pub fn generate_struct_binary_serialize(
    attributes: StructAttributes,
    data: &syn::DataStruct,
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
            let serialize_fields = fields_named.named.iter().filter_map(|f| {
                // Check if the field has attributes
                let opts = FieldAttributes::from_field(f).ok()?;
                let ident = &f.ident;

                // skip file
                if opts.skip.is_some() {
                    None
                } else {
                    Some(quote! {
                        ::binja::serializer::binary_serialize(&self.#ident, serializer)?;
                    })
                }
            });

            quote! {
                #(#serialize_fields)*
            }
        }

        // struct Example(String) , struct Example(String, String)
        syn::Fields::Unnamed(fields_unnamed) => {
            let serialize_fields =
                fields_unnamed
                    .unnamed
                    .iter()
                    .enumerate()
                    .filter_map(|(i, f)| {
                        let opts = FieldAttributes::from_field(f).ok()?;
                        let index = syn::Index::from(i);
                        if opts.skip.is_some() {
                            None
                        } else {
                            Some(quote! {
                            ::binja::serializer::binary_serialize(&self.#index, serializer)?;
                            })
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
    attributes: StructAttributes,
    data: &syn::DataStruct,
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
            let parse_fields = fields_named.named.iter().filter_map(|f| {
                // Check if the field has attributes
                let opts = FieldAttributes::from_field(f).ok()?;
                let ident = &f.ident;

                // skip file
                if opts.skip.is_some() {
                    Some(quote! {
                        #ident: Default::default(),
                    })
                } else {
                    Some(quote! {
                        #ident: ::binja::parser::binary_parse(parser)?,
                    })
                }
            });

            quote! {
                #(#parse_fields)*
            }
        }

        // struct Example(String) , struct Example(String, String)
        syn::Fields::Unnamed(fields_unnamed) => {
            let parse_fields = fields_unnamed
                .unnamed
                .iter()
                .enumerate()
                .filter_map(|(i, f)| {
                    let opts = FieldAttributes::from_field(f).ok()?;
                    let index = syn::Index::from(i);
                    if opts.skip.is_some() {
                        Some(quote! {
                        #index : Default::default(),
                         })
                    } else {
                        Some(quote! {
                        #index : ::binja::parser::binary_parse(parser)?,
                         })
                    }
                });

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
                Ok(Self{
                    #fields_token
                })
            }
        }
    })
}
