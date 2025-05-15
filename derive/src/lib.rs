mod attribute;
mod bit_field;
mod derive_enum;
mod derive_struct;

use attribute::{EnumAttributes, StructAttributes};
use derive_enum::{generate_enum_binary_parse, generate_enum_binary_serialize};
use derive_struct::{generate_struct_binary_parse, generate_struct_binary_serialize};

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(BinarySerialize, attributes(binja))]
pub fn derive_binary_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => {
            let attr = match StructAttributes::from_derive_input(&input) {
                Ok(c) => c,
                Err(e) => return e.write_errors().into(),
            };

            generate_struct_binary_serialize(data, &attr)
        }
        syn::Data::Enum(data) => {
            let attr = match EnumAttributes::from_derive_input(&input) {
                Ok(c) => c,
                Err(e) => return e.write_errors().into(),
            };
            generate_enum_binary_serialize(data, &attr)
        }
        syn::Data::Union(_) => todo!(),
    }
}

#[proc_macro_derive(BinaryParse, attributes(binja))]
pub fn derive_binary_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => {
            let attr = match StructAttributes::from_derive_input(&input) {
                Ok(c) => c,
                Err(e) => return e.write_errors().into(),
            };

            generate_struct_binary_parse(data, &attr)
        }
        syn::Data::Enum(data) => {
            let attr = match EnumAttributes::from_derive_input(&input) {
                Ok(c) => c,
                Err(e) => return e.write_errors().into(),
            };
            generate_enum_binary_parse(data, &attr)
        }
        syn::Data::Union(_) => todo!(),
    }
}
