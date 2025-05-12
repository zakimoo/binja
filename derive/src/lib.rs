mod attribute;
mod derive_enum;
mod derive_struct;

use derive_enum::BinjaEnumOpts;
use derive_struct::BinjaStructOpts;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(BinarySerialize, attributes(binja))]
pub fn derive_binary_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => {
            let container = match BinjaStructOpts::from_derive_input(&input) {
                Ok(c) => c,
                Err(e) => return e.write_errors().into(),
            };

            container.generate_binary_serialize(data)
        }
        syn::Data::Enum(data) => {
            let container = match BinjaEnumOpts::from_derive_input(&input) {
                Ok(c) => c,
                Err(e) => return e.write_errors().into(),
            };
            container.generate_binary_serialize(data)
        }
        syn::Data::Union(_) => todo!(),
    }
}

#[proc_macro_derive(BinaryParse, attributes(binja))]
pub fn derive_binary_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => {
            let container = match BinjaStructOpts::from_derive_input(&input) {
                Ok(c) => c,
                Err(e) => return e.write_errors().into(),
            };

            container.generate_binary_parse(data)
        }
        syn::Data::Enum(data) => {
            // let container = match BinjaEnumOpts::from_derive_input(&input) {
            //     Ok(c) => c,
            //     Err(e) => return e.write_errors().into(),
            // };
            // container.generate_binary_parse(data)

            todo!()
        }
        syn::Data::Union(_) => todo!(),
    }
}

// let fields = if let syn::Data::Struct(syn::DataStruct {
//     fields: syn::Fields::Named(ref fields),
//     ..
// }) = input.data
// {
//     fields
// } else {
//     return syn::Error::new_spanned(input, "Only named fields are supported")
//         .to_compile_error()
//         .into();
// };

// let serialize_fields = fields.named.iter().filter_map(|f| {
//     let opts = BinjaFieldOpts::from_field(f).ok()?;

//     let ident = &f.ident;
//     if opts.skip.is_some() {
//         None
//     } else {
//         Some(quote! {
//             self.#ident.binary_serialize(serializer)?;
//         })
//     }
// });

// let name = &container.ident;

// TokenStream::from(quote! {
//     impl binja::serializer::BinarySerialize for #name {
//         fn binary_serialize(&self, serializer: &mut binja::serializer::BinarySerializer) -> binja::error::Result<()> {
//             #(#serialize_fields)*
//             Ok(())
//         }
//     }
// })
