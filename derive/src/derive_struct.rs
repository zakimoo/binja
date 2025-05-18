use darling::FromField;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{parse_quote, spanned::Spanned};

use crate::{
    attribute::{FieldAttributes, StructAttributes},
    bit_field::{flush_bit_field_at_end, flush_bit_field_if_needed, gen_bit_field_serialization},
};

pub const UNNAMED_FIELD_PREFIX: &str = "field_";

pub fn generate_struct_binary_serialize(
    data: &syn::DataStruct,
    attributes: &StructAttributes,
) -> syn::Result<TokenStream> {
    let struct_name: &syn::Ident = &attributes.ident;
    let generics = &attributes.generics;
    let mut gen_clone = generics.clone();

    // Add trait bounds to each type parameter
    let where_clause = gen_clone.make_where_clause();
    for param in generics.type_params() {
        let ident = &param.ident;
        where_clause.predicates.push(parse_quote! {
            #ident: ::binja::BinarySerialize
        });
    }

    let fields_token = match &data.fields {
        // struct Example { field: String }
        syn::Fields::Named(fields_named) => {
            let (fields_names, field_ser_code) = gen_ser_fields(&fields_named.named)?;

            quote! {
               let #struct_name { #fields_names } = self;
                #field_ser_code
            }
        }
        // struct Example(String) , struct Example(String, String)
        syn::Fields::Unnamed(fields_unnamed) => {
            let (fields_names, field_ser_code) = gen_ser_fields(&fields_unnamed.unnamed)?;

            quote! {
                let #struct_name ( #fields_names ) = self;
                #field_ser_code
            }
        }
        // struct Example;
        syn::Fields::Unit => {
            quote! {}
        }
    };

    let expand = quote! {
        #[allow(unused_variables)]
        impl #generics ::binja::BinarySerialize for #struct_name #generics #where_clause{
            fn binary_serialize(&self, serializer: &mut ::binja::BinarySerializer) -> ::binja::error::Result<()> {
                #fields_token
                Ok(())
            }
        }
    };

    Ok(expand.into())
}

pub fn generate_struct_binary_parse(
    data: &syn::DataStruct,
    attributes: &StructAttributes,
) -> syn::Result<TokenStream> {
    let name = &attributes.ident;
    let generics = &attributes.generics;
    let mut gen_clone = generics.clone();

    // Add trait bounds to each type parameter
    let where_clause = gen_clone.make_where_clause();
    for param in generics.type_params() {
        let ident = &param.ident;
        where_clause.predicates.push(parse_quote! {
            #ident: ::binja::par::BinaryParse
        });
    }

    let code = match &data.fields {
        // struct Example { field: String }
        syn::Fields::Named(fields_named) => {
            let (fields_names, fields_par_code) = gen_par_fields(&fields_named.named)?;
            quote! {
                #fields_par_code
                Ok(Self {
                    #fields_names
                })
            }
        }

        // struct Example(String) , struct Example(String, String)
        syn::Fields::Unnamed(fields_unnamed) => {
            let (fields_names, fields_par_code) = gen_par_fields(&fields_unnamed.unnamed)?;
            quote! {
                #fields_par_code
                Ok(Self(
                    #fields_names
                ))
            }
        }

        // struct Example;
        syn::Fields::Unit => quote! {Ok(Self {})},
    };

    let expand = quote! {
        impl #generics binja::par::BinaryParse for #name #generics #where_clause{
            fn binary_parse(parser: &mut ::binja::par::BinaryParser) -> binja::error::Result<Self> {
                #code
            }
        }
    };

    Ok(expand.into())
}

pub fn gen_ser_fields(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> syn::Result<(TokenStream, TokenStream)> {
    let mut field_names = Vec::new();
    let mut code = Vec::new();
    let mut bit_field_declared = false;
    let mut bit_offset = 0u8;

    for (i, f) in fields.iter().enumerate() {
        // Check if the field has attributes
        let attrs = FieldAttributes::from_field(f)?;
        attrs.validate(f.span())?;

        let field_expr = get_field_expr(f, i);
        field_names.push(field_expr.clone());

        // skip field
        if attrs.skip() {
            continue;
        }

        // if field have #[binja(bits = 6)]
        if let Some(bits) = attrs.bits() {
            // only construct bit field if it is not already constructed
            // if there is a field with bits
            if !bit_field_declared {
                code.push(quote! { let mut bit_field = 0u8; });
                bit_field_declared = true;
            }

            if is_type_bool(&f.ty) {
                code.push(quote! {
                    let #field_expr = if *#field_expr {1usize} else {0usize};
                });
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
                ::binja::BinarySerialize::binary_serialize(#field_expr, serializer)?;
            })
        }
    }

    // if last field is a bit field smaller that 8 bits
    flush_bit_field_at_end(&mut code, bit_offset);

    Ok((
        quote! {
            #(#field_names),*
            ,..
        },
        quote! {
             #(#code)*
        },
    ))
}

pub fn gen_par_fields(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> syn::Result<(TokenStream, TokenStream)> {
    let mut code = Vec::new();
    let mut fields_names = Vec::new();

    let mut bit_offset: u8 = 0;
    let byte_var = quote! { bit_field };

    for (i, f) in fields.iter().enumerate() {
        let attrs = FieldAttributes::from_field(f)?;
        attrs.validate(f.span())?;

        let ident = get_field_expr(f, i);
        let is_bool = is_type_bool(&f.ty);
        let field_type = if is_bool {
            quote! { usize }
        } else {
            let ty = &f.ty;
            quote! { #ty }
        };
        fields_names.push(ident.clone());

        if attrs.skip() {
            code.push(quote! {
                let #ident = Default::default();
            });
            continue;
        }

        if let Some(bits) = attrs.bits() {
            let mut remaining_bits = bits;
            let mut local_shift = 0u8;

            // Define the field variable
            code.push(quote! {
                let mut #ident: #field_type = 0;
            });

            while remaining_bits > 0 {
                let bits_in_this_byte = 8 - (bit_offset % 8);
                let consume_bits = remaining_bits.min(bits_in_this_byte);

                // Read a new byte if starting fresh or if no byte is loaded yet
                if bit_offset % 8 == 0 {
                    code.push(quote! {
                        let #byte_var: u8 = ::binja::par::binary_parse(parser)?;
                    });
                }

                let right_shift = bit_offset % 8;

                let mut expr = quote! {(#byte_var as #field_type)};

                expr = if right_shift > 0 {
                    quote! { (#expr >> #right_shift) }
                } else {
                    expr
                };

                expr = if consume_bits == 8 || consume_bits + right_shift == 8 {
                    expr
                } else {
                    quote! { (#expr &::binja::bit_mask!(#consume_bits)) }
                };

                expr = if local_shift > 0 {
                    quote! {( #expr << #local_shift )}
                } else {
                    expr
                };

                code.push(quote! {
                    #ident |= #expr;
                });

                bit_offset += consume_bits;
                remaining_bits -= consume_bits;
                local_shift += consume_bits;
            }

            if is_bool {
                code.push(quote! {
                  let  #ident = if #ident == 0 { false } else { true };
                });
            }
        } else {
            // if last field is a bit field smaller that 8 bits
            // current field is not a bit field
            bit_offset = 0;

            code.push(quote! {
                let #ident = ::binja::par::binary_parse(parser)?;
            });
        }
    }

    Ok((
        quote! {
            #(#fields_names),*
        },
        quote! {
            #(#code)*
        },
    ))
}

pub fn get_field_expr(f: &syn::Field, i: usize) -> TokenStream {
    if let Some(ident) = &f.ident {
        return ident.clone().into_token_stream();
    }

    syn::Ident::new(&format!("{UNNAMED_FIELD_PREFIX}{i}"), Span::call_site()).into_token_stream()
}

pub fn is_type_bool(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        if let Some(ident) = path.get_ident() {
            return ident == "bool";
        }
    }
    false
}
