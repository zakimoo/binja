use std::vec;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, parse_quote, token::Eq};

use crate::{
    attribute::EnumAttributes,
    derive_struct::{
        generate_parse_named_fields, generate_parse_unnamed_fields,
        generate_serialize_named_fields, generate_serialize_unnamed_fields,
    },
};

pub fn generate_enum_binary_serialize(data: &syn::DataEnum, attr: &EnumAttributes) -> TokenStream {
    let name = &attr.ident;
    let generics = &attr.generics;
    let mut gen_clone = generics.clone();

    // Add trait bounds to each type parameter
    let where_clause = gen_clone.make_where_clause();
    for param in generics.type_params() {
        let ident = &param.ident;
        where_clause.predicates.push(parse_quote! {
            #ident: ::binja::serializer::BinarySerialize
        });
    }

    let variant_arms = generate_enum_serialize_variants(&data.variants, attr);

    TokenStream::from(quote! {
        impl #generics binja::serializer::BinarySerialize for #name #generics #where_clause{
            fn binary_serialize(&self, serializer: &mut binja::serializer::BinarySerializer) -> binja::error::Result<()> {
                match self {
                    #(#variant_arms),*
                }
                Ok(())
            }
        }
    })
}

fn generate_enum_serialize_variants(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
    attr: &EnumAttributes,
) -> Vec<proc_macro2::TokenStream> {
    let repr = attr.repr();
    let untagged = attr.untagged();
    let mut current_value: isize = -1;

    variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;

            get_enum_value(&mut current_value, &variant.discriminant);

            // Create a literal with the correct suffix (e.g., 1i8)
            let v_lit = syn::LitInt::new(
                &format!("{}{}", current_value, repr),
                proc_macro2::Span::call_site(),
            );

            // enum TestEnum {
            //     AA,
            //     A = 10,
            //     B,
            //     C(u8) = 20,
            //     D { a: u8, b: i16 },
            //     E(u8, i16),
            // }
            let (pat, serialize_fields) = match &variant.fields {
                syn::Fields::Named(fields) => {
                    let field_names: Vec<_> = fields
                        .named
                        .iter()
                        .map(|f| f.ident.as_ref().unwrap())
                        .collect();

                    // Self::D { a, b }
                    let pats = quote! { { #(#field_names),* } };

                    // code to run
                    let ser = generate_serialize_named_fields(fields, false);

                    (pats, ser)
                }
                syn::Fields::Unnamed(fields) => {
                    let idents: Vec<syn::Ident> = (0..fields.unnamed.len())
                        .map(|i| {
                            syn::Ident::new(&format!("field_{i}"), proc_macro2::Span::call_site())
                        })
                        .collect();
                    // Self::C(field_0)
                    let pats = quote! { ( #(#idents),* ) };
                    // code to run
                    let ser = generate_serialize_unnamed_fields(fields, false);
                    (pats, ser)
                }

                syn::Fields::Unit => (quote! {}, vec![quote! {}]),
            };

            let discriminant_code = if untagged {
                quote! {}
            } else {
                quote! {
                    let value = #v_lit;
                    ::binja::serializer::binary_serialize(&value, serializer)?;
                }
            };

            quote! {
                Self::#variant_ident #pat => {
                    #discriminant_code
                    #(#serialize_fields)*
                }
            }
        })
        .collect()
}

pub fn generate_enum_binary_parse(data: &syn::DataEnum, attr: &EnumAttributes) -> TokenStream {
    let name = &attr.ident;
    let generics = &attr.generics;
    let mut gen_clone = generics.clone();

    // Add trait bounds to each type parameter
    let where_clause = gen_clone.make_where_clause();
    for param in generics.type_params() {
        let ident = &param.ident;
        where_clause.predicates.push(parse_quote! {
            #ident: ::binja::parser::BinaryParse
        });
    }

    let parse_code = if attr.untagged() {
        parse_untagged_enum(&data.variants)
    } else {
        parse_tagged_enum(&data.variants, attr.repr())
    };

    TokenStream::from(quote! {
        impl #generics binja::parser::BinaryParse for #name #generics #where_clause{
            fn binary_parse(parser: &mut binja::parser::BinaryParser) -> binja::error::Result<Self> {
                #parse_code
            }
        }
    })
}

fn parse_untagged_enum(
    variant: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    variant
        .first()
        .map(|variant| {
            let variant_ident = &variant.ident;

            let parsers = match &variant.fields {
                syn::Fields::Unit => {
                    vec![quote! {}]
                }
                syn::Fields::Unnamed(fields) => generate_parse_unnamed_fields(fields, false),
                syn::Fields::Named(fields) => generate_parse_named_fields(fields, false),
            };

            quote! {
              Ok(Self::#variant_ident {
                #(#parsers)*
              })
            }
        })
        .unwrap_or_else(|| {
            quote! {
                Ok(Self)
            }
        })
}

fn parse_tagged_enum(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
    repr: String,
) -> proc_macro2::TokenStream {
    let mut current_value: isize = -1;
    let mut seen_values = vec![];
    let variant_arms = variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;

            get_enum_value(&mut current_value, &variant.discriminant);

            // Create a literal with the correct suffix (e.g., 1i8)
            let v_lit = syn::LitInt::new(
                &format!("{}{}", current_value, repr),
                proc_macro2::Span::call_site(),
            );

            seen_values.push(current_value);

            match &variant.fields {
                syn::Fields::Unit => quote! {
                    #v_lit => Ok(Self::#variant_ident),
                },
                syn::Fields::Unnamed(fields) => {
                    let parsers = generate_parse_unnamed_fields(fields, false);
                    quote! {
                        #v_lit => Ok(Self::#variant_ident{
                            #(#parsers)*
                        }),
                    }
                }
                syn::Fields::Named(fields) => {
                    let parsers = generate_parse_named_fields(fields, false);
                    quote! {
                        #v_lit => Ok(Self::#variant_ident {
                            #(#parsers)*
                        }),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    // Format expected values as a human-readable string
    let mut expected = seen_values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();
    let expected_str = match expected.len() {
        0 => "".to_string(),
        1 => expected[0].clone(),
        _ => {
            let last = expected.pop().unwrap();
            format!("{} or {}", expected.join(", "), last)
        }
    };

    let repr_ty: syn::Type = syn::parse_str(&repr).unwrap();

    quote! {
        let current_value: #repr_ty = ::binja::parser::binary_parse(parser)?;
        match current_value{
            #(#variant_arms)*
            x => Err(::binja::error::Error::InvalidVariant {
                expected: #expected_str.to_string(),
                found: format!("{}", x),
            }),
        }
    }
}

fn get_enum_value(current_value: &mut isize, discriminant: &Option<(Eq, Expr)>) {
    if let Some((_, expr)) = discriminant {
        match expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(lit_int),
                ..
            }) => {
                let value = lit_int.base10_parse::<isize>().unwrap();
                *current_value = value;
            }
            syn::Expr::Unary(syn::ExprUnary {
                op: syn::UnOp::Neg(_),
                expr,
                ..
            }) => {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit_int),
                    ..
                }) = &**expr
                {
                    let value = lit_int.base10_parse::<isize>().unwrap();
                    *current_value = -value;
                } else {
                    panic!("Unsupported expression in unary negation");
                }
            }
            _ => {
                panic!("Only literal integer discriminants are supported for now");
            }
        }
    } else {
        // If no discriminant is provided, use the current value
        *current_value += 1;
    }
}
