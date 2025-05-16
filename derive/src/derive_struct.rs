use darling::FromField;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, spanned::Spanned};

use crate::{
    attribute::{FieldAttributes, StructAttributes},
    bit_field::{flush_bit_field_at_end, flush_bit_field_if_needed, gen_bit_field_serialization},
};

pub fn generate_struct_binary_serialize(
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
            #ident: ::binja::serializer::BinarySerialize
        });
    }

    let fields_token = match &data.fields {
        // struct Example { field: String }
        syn::Fields::Named(fields_named) => {
            gen_ser_fields(&fields_named.named, |f, _| {
                let ident = &f.ident;
                // struct Example { field: String }
                // ::binja::serializer::binary_serialize(&self.field, serializer)?;
                quote! { &self.#ident }
            })?
        }
        // struct Example(String) , struct Example(String, String)
        syn::Fields::Unnamed(fields_unnamed) => {
            gen_ser_fields(&fields_unnamed.unnamed, |_, i| {
                let index = syn::Index::from(i);
                // struct Example(u8)
                // ::binja::serializer::binary_serialize(&self.0, serializer)?;
                quote! {
                    &self.#index
                }
            })?
        }
        // struct Example;
        syn::Fields::Unit => {
            quote! {}
        }
    };

    let expand = quote! {
        impl #generics binja::serializer::BinarySerialize for #name #generics #where_clause{
            fn binary_serialize(&self, serializer: &mut ::binja::serializer::BinarySerializer) -> binja::error::Result<()> {
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
        )?,

        // struct Example(String) , struct Example(String, String)
        syn::Fields::Unnamed(fields_unnamed) => gen_par_fields(
            &fields_unnamed.unnamed,
            |_, i| {
                let ident = syn::Ident::new(&format!("field_{i}"), Span::call_site());
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
        )?,

        // struct Example;
        syn::Fields::Unit => quote! {Ok(Self {})},
    };

    let expand = quote! {
        impl #generics binja::parser::BinaryParse for #name #generics #where_clause{
            fn binary_parse(parser: &mut ::binja::parser::BinaryParser) -> binja::error::Result<Self> {
                #code
            }
        }
    };

    Ok(expand.into())
}

pub fn gen_ser_fields<F>(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    get_field_expr: F,
) -> syn::Result<TokenStream>
where
    F: Fn(&syn::Field, usize) -> TokenStream,
{
    let mut code = Vec::new();
    let mut bit_field_declared = false;
    let mut bit_offset = 0u8;

    for (i, f) in fields.iter().enumerate() {
        // Check if the field has attributes
        let attrs = FieldAttributes::from_field(f)?;
        attrs.validate(f.span())?;

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

    Ok(quote! {
         #(#code)*
    })
}

pub fn gen_par_fields<F1, F2>(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    get_field_expr: F1,
    get_return_code: F2,
) -> syn::Result<TokenStream>
where
    F1: Fn(&syn::Field, usize) -> TokenStream,
    F2: Fn(&TokenStream) -> TokenStream,
{
    let mut code = Vec::new();
    let mut fields_code = Vec::new();

    let mut bit_offset: u8 = 0;
    let byte_var = quote! { bit_field };

    for (i, f) in fields.iter().enumerate() {
        let attrs = FieldAttributes::from_field(f)?;
        attrs.validate(f.span())?;

        let ident = get_field_expr(f, i);
        let field_type = &f.ty;
        fields_code.push(ident.clone());

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
                        let #byte_var: u8 = ::binja::parser::binary_parse(parser)?;
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
        } else {
            // if last field is a bit field smaller that 8 bits
            // current field is not a bit field
            bit_offset = 0;

            code.push(quote! {
                let #ident = ::binja::parser::binary_parse(parser)?;
            });
        }
    }

    let return_code = get_return_code(&quote! {
        #(#fields_code),*
    });

    Ok(quote! {
        #(#code)*
        #return_code
    })
}

pub fn is_type_bool(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        if let Some(ident) = path.get_ident() {
            return ident == "bool";
        }
    }
    false
}
