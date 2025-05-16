use std::vec;

use darling::FromVariant;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Expr, parse_quote, token::Eq};

use crate::{
    attribute::{EnumAttributes, VariantAttributes},
    derive_struct::{gen_par_fields, gen_ser_fields},
};

pub fn generate_enum_binary_serialize(
    data: &syn::DataEnum,
    attr: &EnumAttributes,
) -> syn::Result<TokenStream> {
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

    let variant_arms = generate_enum_serialize_variants(&data.variants, attr)?;

    let expand = quote! {
         #[allow(unused_variables)]
        impl #generics binja::serializer::BinarySerialize for #name #generics #where_clause{
            fn binary_serialize(&self, serializer: &mut binja::serializer::BinarySerializer) -> binja::error::Result<()> {
                match self {
                    #variant_arms
                }
                Ok(())
            }
        }
    };

    Ok(TokenStream::from(expand))
}

fn generate_enum_serialize_variants(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
    attr: &EnumAttributes,
) -> syn::Result<TokenStream> {
    let repr = attr.repr();
    let untagged = attr.untagged();
    let mut current_value: isize = -1;

    let mut code = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;

        // leave it here to validate variant attributes
        let _attr = VariantAttributes::from_variant(variant)?;

        get_enum_value(&mut current_value, &variant.discriminant);

        // Create a literal with the correct suffix (e.g., 1i8)
        let v_lit = syn::LitInt::new(&format!("{}{}", current_value, repr), Span::call_site());

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
                // code to run
                let (fields_names, fields_ser_code) = gen_ser_fields(&fields.named)?;

                (quote! {{#fields_names}}, fields_ser_code)
            }
            syn::Fields::Unnamed(fields) => {
                let (fields_names, fields_ser_code) = gen_ser_fields(&fields.unnamed)?;
                (quote! {(#fields_names)}, fields_ser_code)
            }

            syn::Fields::Unit => (quote! {}, quote! {}),
        };

        let discriminant_code = if untagged {
            quote! {}
        } else {
            quote! {
                let value = #v_lit;
                ::binja::serializer::binary_serialize(&value, serializer)?;
            }
        };

        code.push(quote! {
            Self::#variant_ident #pat => {
                #discriminant_code
                #serialize_fields
            }
        });
    }

    let expand = quote! {
        #(#code)*
    };

    Ok(expand)
}

pub fn generate_enum_binary_parse(
    data: &syn::DataEnum,
    attr: &EnumAttributes,
) -> syn::Result<TokenStream> {
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

    let parse_code = gen_par_variants(&data.variants, attr)?;

    let expand = quote! {
        impl #generics binja::parser::BinaryParse for #name #generics #where_clause{
            fn binary_parse(parser: &mut binja::parser::BinaryParser) -> binja::error::Result<Self> {
                #parse_code
            }
        }
    };

    Ok(expand.into())
}

fn gen_par_variants(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
    attrs: &EnumAttributes,
) -> syn::Result<TokenStream> {
    let mut current_value: isize = -1;
    let mut seen_values = vec![];

    let mut variant_arms = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;

        // leave it here to validate variant attributes
        let _attr = VariantAttributes::from_variant(variant)?;

        get_enum_value(&mut current_value, &variant.discriminant);

        // Create a literal with the correct suffix (e.g., 1i8)
        let v_lit = syn::LitInt::new(
            &format!("{}{}", current_value, attrs.repr()),
            Span::call_site(),
        );

        seen_values.push(current_value);

        match &variant.fields {
            syn::Fields::Unit => variant_arms.push(quote! {
                #v_lit => Ok(Self::#variant_ident),
            }),
            syn::Fields::Unnamed(fields) => {
                let (fields_names, fields_code) = gen_par_fields(&fields.unnamed)?;

                variant_arms.push(quote! {
                    #v_lit => {
                        #fields_code
                        Ok(Self::#variant_ident(
                            #fields_names
                        ))
                    }
                });
            }
            syn::Fields::Named(fields) => {
                let (fields_names, fields_code) = gen_par_fields(&fields.named)?;

                variant_arms.push(quote! {
                    #v_lit => {
                        #fields_code
                        Ok(Self::#variant_ident {
                            #fields_names
                        })
                    }
                });
                // let parsers = gen_par_named_fields(fields, false);
                // quote! {
                //     #v_lit => Ok(Self::#variant_ident {
                //         #(#parsers)*
                //     }),
                // }
            }
        }
    }

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

    let repr_ty: syn::Type = syn::parse_str(&attrs.repr()).unwrap();

    let current_value_code = if attrs.untagged() {
        let v_lit = syn::LitInt::new(
            &format!("{}{}", seen_values.first().unwrap_or(&0), attrs.repr()),
            Span::call_site(),
        );
        quote! {
            let current_value: #repr_ty = #v_lit;
        }
    } else {
        quote! {
            let current_value: #repr_ty = ::binja::parser::binary_parse(parser)?;
        }
    };

    let expand = quote! {
        #current_value_code
        match current_value{
            #(#variant_arms)*
            x => Err(::binja::error::Error::InvalidVariant {
                expected: #expected_str.to_string(),
                found: format!("{}", x),
            }),
        }
    };

    Ok(expand)
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
