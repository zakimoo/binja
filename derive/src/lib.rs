mod attribute;
mod derive_enum;
mod derive_struct;

use attribute::ContainerAttributes;
use virtue::prelude::*;

#[proc_macro_derive(BinarySerialize, attributes(binja))]
pub fn derive_your_derive(input: TokenStream) -> TokenStream {
    derive_your_derive_inner(input).unwrap_or_else(|error| error.into_token_stream())
}

fn derive_your_derive_inner(input: TokenStream) -> Result<TokenStream> {
    // Parse the struct or enum you want to implement a derive for
    let parse = Parse::new(input)?;
    // Get a reference to the generator
    let (mut generator, attr, body) = parse.into_generator();

    let attributes = attr
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
                attributes,
            }
            .generate_binary_serialize(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_binary_serialize(&mut generator)?;
        }
    }
    generator.export_to_file("binja", "BinarySerialize");
    generator.finish()
}

#[proc_macro_derive(BinaryParse, attributes(some, attributes, go, here))]
pub fn derive_binary_parse(input: TokenStream) -> TokenStream {
    derive_binary_parse_inner(input).unwrap_or_else(|error| error.into_token_stream())
}

fn derive_binary_parse_inner(input: TokenStream) -> Result<TokenStream> {
    // Parse the struct or enum you want to implement a derive for
    let parse = Parse::new(input)?;
    // Get a reference to the generator
    let (mut generator, attr, body) = parse.into_generator();

    let attributes = attr
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
                attributes,
            }
            .generate_binary_parse(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_binary_parse(&mut generator)?;
        }
    }
    generator.export_to_file("binja", "BinaryParse");
    generator.finish()
}
