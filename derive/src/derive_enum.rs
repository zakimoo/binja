use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, parse_quote, token::Eq};

use crate::attribute::EnumAttributes;

pub fn generate_enum_binary_serialize(attr: EnumAttributes, data: &syn::DataEnum) -> TokenStream {
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

    let repr_ty: syn::Type = syn::parse_str(&attr.repr()).unwrap();
    let untagged = attr.untagged();

    let mut current_value: isize = 0;

    let variant_arms = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        get_enum_value(&mut current_value, &variant.discriminant);

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
                let ser = quote! {
                    #(
                         ::binja::serializer::binary_serialize(#field_names, serializer)?;
                    )*
                };

                (pats, ser)
            }
            syn::Fields::Unnamed(fields) => {
                let idents: Vec<syn::Ident> = (0..fields.unnamed.len())
                    .map(|i| syn::Ident::new(&format!("field_{i}"), proc_macro2::Span::call_site()))
                    .collect();
                // Self::C(field_0)
                let pats = quote! { ( #(#idents),* ) };
                // code to run
                let ser = quote! {
                    #(
                        ::binja::serializer::binary_serialize(#idents, serializer)?;
                    )*
                };
                (pats, ser)
            }

            syn::Fields::Unit => (quote! {}, quote! {}),
        };

        let discriminant_code = if untagged {
            quote! {}
        } else {
            quote! {
                let value: #repr_ty = #current_value as #repr_ty;
                ::binja::serializer::binary_serialize(&value, serializer)?;
            }
        };

        current_value += 1;

        quote! {
            Self::#variant_ident #pat => {
                #discriminant_code
                #serialize_fields
            }
        }
    });

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
    }
}
