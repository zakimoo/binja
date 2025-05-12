use crate::attribute::ContainerAttributes;
use virtue::prelude::*;

const TUPLE_FIELD_PREFIX: &str = "field_";
const DEFAULT_ENUM_REPR: &str = "u32";
pub(crate) struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
    pub attributes: ContainerAttributes,
}

impl DeriveEnum {
    pub fn generate_binary_serialize(self, generator: &mut Generator) -> Result<()> {
        let crate_name = self.attributes.crate_name.as_str();
        // use #[repr(x)]
        let repr = self
            .attributes
            .repr
            .unwrap_or(DEFAULT_ENUM_REPR.to_string());

        // let untagged = self.attributes.untagged;

        println!("repr: {}", repr);
        // println!("untagged: {}", untagged);
        generator
            .impl_for(format!("{}::serializer::BinarySerialize", crate_name))
            .modify_generic_constraints(|generics, where_constraints| {
                for g in generics.iter_generics() {
                    where_constraints
                        .push_constraint(g, format!("{}::serializer::BinarySerialize", crate_name))
                        .unwrap();
                }
                Ok(())
            })?
            .generate_fn("binary_serialize")
            .with_self_arg(virtue::generate::FnSelfArg::RefSelf)
            .with_arg(
                "serializer",
                format!("&mut {}::serializer::BinarySerializer", crate_name),
            )
            .with_return_type(format!("{}::error::Result<()>", crate_name))
            .body(|fn_body| {
                fn_body.ident_str("match");
                fn_body.ident_str("self");
                fn_body.group(Delimiter::Brace, |match_body| {
                    // used to serialize the value of the enum
                    // if no value is set, we use the variant index
                    let mut current_value: isize = 0;
                    for variant in &self.variants {
                        let variant_value = if let Some(v) = &variant.value {
                            isize::from_str_radix(&v.to_string(), 10).unwrap()
                        } else {
                            current_value
                        };
                        // Self::Variant
                        match_body.ident_str("Self");
                        match_body.puncts("::");
                        match_body.ident(variant.name.clone());

                        // if we have any fields, declare them here
                        // Self::Variant { a, b, c }
                        if let Some(fields) = variant.fields.as_ref() {
                            let delimiter = fields.delimiter();
                            match_body.group(delimiter, |field_body| {
                                for (idx, field_name) in fields.names().into_iter().enumerate() {
                                    if idx != 0 {
                                        field_body.punct(',');
                                    }
                                    field_body.push(
                                        field_name.to_token_tree_with_prefix(TUPLE_FIELD_PREFIX),
                                    );
                                }
                                Ok(())
                            })?;
                        }

                        match_body.puncts("=>");

                        match_body.group(Delimiter::Brace, |body| {
                            // if !untagged {
                            body.push_parsed(format!("let value: {} = {};", repr, variant_value))?;
                            body.push_parsed(format!(
                                "{}::serializer::binary_serialize(&value , serializer)?;",
                                crate_name
                            ))?;
                            // }

                            // If we have any fields, encode them all one by one
                            if let Some(fields) = variant.fields.as_ref() {
                                for field_name in fields.names() {
                                    // let attributes = field_name
                                    //     .attributes()
                                    //     .get_attribute::<FieldAttributes>()?
                                    //     .unwrap_or_default();

                                    body.push_parsed(format!(
                                        "{0}::serializer::binary_serialize({1}, serializer)?;",
                                        crate_name,
                                        field_name.to_string_with_prefix(TUPLE_FIELD_PREFIX),
                                    ))?;
                                }
                            }
                            Ok(())
                        })?;
                        match_body.punct(',');
                        current_value = variant_value + 1;
                    }
                    Ok(())
                })?;
                fn_body.push_parsed("Ok(())")?;
                Ok(())
            })?;
        Ok(())
    }

    /// If we're encoding an empty enum, we need to add an empty case in the form of:
    /// `_ => core::unreachable!(),`
    fn encode_empty_enum_case(&self, builder: &mut StreamBuilder) -> Result {
        builder.push_parsed("_ => core::unreachable!()").map(|_| ())
    }

    // /// Build the catch-all case for an int-to-enum decode implementation
    // fn invalid_variant_case(&self, enum_name: &str, result: &mut StreamBuilder) -> Result {
    //     let crate_name = self.attributes.crate_name.as_str();

    //     // we'll be generating:
    //     // variant => Err(
    //     //    bincode::error::DecodeError::UnexpectedVariant {
    //     //        found: variant,
    //     //        type_name: <enum_name>
    //     //        allowed: ...,
    //     //    }
    //     // )
    //     //
    //     // Where allowed is either:
    //     // - bincode::error::AllowedEnumVariants::Range { min: 0, max: <max> }
    //     //   if we have no fixed value variants
    //     // - bincode::error::AllowedEnumVariants::Allowed(&[<variant1>, <variant2>, ...])
    //     //   if we have fixed value variants
    //     result.ident_str("variant");
    //     result.puncts("=>");
    //     result.push_parsed("core::result::Result::Err")?;
    //     result.group(Delimiter::Parenthesis, |err_inner| {
    //         err_inner.push_parsed(format!(
    //             "{}::error::DecodeError::UnexpectedVariant",
    //             crate_name
    //         ))?;
    //         err_inner.group(Delimiter::Brace, |variant_inner| {
    //             variant_inner.ident_str("found");
    //             variant_inner.punct(':');
    //             variant_inner.ident_str("variant");
    //             variant_inner.punct(',');

    //             variant_inner.ident_str("type_name");
    //             variant_inner.punct(':');
    //             variant_inner.lit_str(enum_name);
    //             variant_inner.punct(',');

    //             variant_inner.ident_str("allowed");
    //             variant_inner.punct(':');

    //             if self.variants.iter().any(|i| i.value.is_some()) {
    //                 // we have fixed values, implement AllowedEnumVariants::Allowed
    //                 variant_inner.push_parsed(format!(
    //                     "&{}::error::AllowedEnumVariants::Allowed",
    //                     crate_name
    //                 ))?;
    //                 variant_inner.group(Delimiter::Parenthesis, |allowed_inner| {
    //                     allowed_inner.punct('&');
    //                     allowed_inner.group(Delimiter::Bracket, |allowed_slice| {
    //                         for (idx, (ident, _)) in self.iter_fields().enumerate() {
    //                             if idx != 0 {
    //                                 allowed_slice.punct(',');
    //                             }
    //                             allowed_slice.extend(ident);
    //                         }
    //                         Ok(())
    //                     })?;
    //                     Ok(())
    //                 })?;
    //             } else {
    //                 // no fixed values, implement a range
    //                 variant_inner.push_parsed(format!(
    //                     "&{0}::error::AllowedEnumVariants::Range {{ min: 0, max: {1} }}",
    //                     crate_name,
    //                     self.variants.len() - 1
    //                 ))?;
    //             }
    //             Ok(())
    //         })?;
    //         Ok(())
    //     })?;
    //     Ok(())
    // }

    pub fn generate_binary_parse(self, generator: &mut Generator) -> Result<()> {
        //     let crate_name = self.attributes.crate_name.as_str();

        //     let decode_context = if let Some((decode_context, _)) = &self.attributes.decode_context {
        //         decode_context.as_str()
        //     } else {
        //         "__Context"
        //     };
        //     // Remember to keep this mostly in sync with generate_borrow_decode

        //     let enum_name = generator.target_name().to_string();

        //     let mut impl_for = generator.impl_for(format!("{}::Decode", crate_name));

        //     if self.attributes.decode_context.is_none() {
        //         impl_for = impl_for.with_impl_generics(["__Context"]);
        //     }

        //     impl_for
        //         .with_trait_generics([decode_context])
        //         .modify_generic_constraints(|generics, where_constraints| {
        //             if let Some((bounds, lit)) = (self.attributes.decode_bounds.as_ref()).or(self.attributes.bounds.as_ref()) {
        //                 where_constraints.clear();
        //                 where_constraints.push_parsed_constraint(bounds).map_err(|e| e.with_span(lit.span()))?;
        //             } else {
        //                 for g in generics.iter_generics() {
        //                     where_constraints.push_constraint(g, format!("{}::Decode<__Context>", crate_name))?;
        //                 }
        //             }
        //             Ok(())
        //         })?
        //         .generate_fn("decode")
        //         .with_generic_deps("__D", [format!("{}::de::Decoder<Context = {}>", crate_name, decode_context)])
        //         .with_arg("decoder", "&mut __D")
        //         .with_return_type(format!("core::result::Result<Self, {}::error::DecodeError>", crate_name))
        //         .body(|fn_builder| {
        //             if self.variants.is_empty() {
        //                 fn_builder.push_parsed(format!(
        //                     "core::result::Result::Err({}::error::DecodeError::EmptyEnum {{ type_name: core::any::type_name::<Self>() }})",
        //                     crate_name
        //                 ))?;
        //             } else {
        //                 fn_builder
        //                     .push_parsed(format!(
        //                         "let variant_index = <u32 as {}::Decode::<__D::Context>>::decode(decoder)?;",
        //                         crate_name
        //                     ))?;
        //                 fn_builder.push_parsed("match variant_index")?;
        //                 fn_builder.group(Delimiter::Brace, |variant_case| {
        //                     for (mut variant_index, variant) in self.iter_fields() {
        //                         // idx => Ok(..)
        //                         if variant_index.len() > 1 {
        //                             variant_case.push_parsed("x if x == ")?;
        //                             variant_case.extend(variant_index);
        //                         } else {
        //                             variant_case.push(variant_index.remove(0));
        //                         }
        //                         variant_case.puncts("=>");
        //                         variant_case.push_parsed("core::result::Result::Ok")?;
        //                         variant_case.group(Delimiter::Parenthesis, |variant_case_body| {
        //                             // Self::Variant { }
        //                             // Self::Variant { 0: ..., 1: ... 2: ... },
        //                             // Self::Variant { a: ..., b: ... c: ... },
        //                             variant_case_body.ident_str("Self");
        //                             variant_case_body.puncts("::");
        //                             variant_case_body.ident(variant.name.clone());

        //                             variant_case_body.group(Delimiter::Brace, |variant_body| {
        //                                 if let Some(fields) = variant.fields.as_ref() {
        //                                     let is_tuple = matches!(fields, Fields::Tuple(_));
        //                                     for (idx, field) in fields.names().into_iter().enumerate() {
        //                                         if is_tuple {
        //                                             variant_body.lit_usize(idx);
        //                                         } else {
        //                                             variant_body.ident(field.unwrap_ident().clone());
        //                                         }
        //                                         variant_body.punct(':');
        //                                         let attributes = field.attributes().get_attribute::<FieldAttributes>()?.unwrap_or_default();
        //                                         if attributes.with_serde {
        //                                             variant_body
        //                                                 .push_parsed(format!(
        //                                                     "<{0}::serde::Compat<_> as {0}::Decode::<__D::Context>>::decode(decoder)?.0,",
        //                                                     crate_name
        //                                                 ))?;
        //                                         } else {
        //                                             variant_body
        //                                                 .push_parsed(format!(
        //                                                     "{}::Decode::<__D::Context>::decode(decoder)?,",
        //                                                     crate_name
        //                                                 ))?;
        //                                         }
        //                                     }
        //                                 }
        //                                 Ok(())
        //                             })?;
        //                             Ok(())
        //                         })?;
        //                         variant_case.punct(',');
        //                     }

        //                     // invalid idx
        //                     self.invalid_variant_case(&enum_name, variant_case)
        //                 })?;
        //             }
        //             Ok(())
        //         })?;
        //     self.generate_borrow_decode(generator)?;
        Ok(())
    }
}
