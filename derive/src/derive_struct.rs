use std::any::Any;

use crate::attribute::{ContainerAttributes, FieldAttributes};
use virtue::prelude::*;

pub(crate) struct DeriveStruct {
    pub fields: Option<Fields>,
    pub attributes: ContainerAttributes,
}

impl DeriveStruct {
    pub fn generate_binary_serialize(self, generator: &mut Generator) -> Result<()> {
        let crate_name = &self.attributes.crate_name;

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
                if let Some(fields) = self.fields.as_ref() {
                    for field in fields.names() {
                        // TODO: handle attributes like skip, default, etc.
                        // let _attributes = field
                        //     .attributes()
                        //     .get_attribute::<FieldAttributes>()?
                        //     .unwrap_or_default();

                        fn_body.push_parsed(format!(
                            "{}::serializer::binary_serialize(&self.{}, serializer)?;",
                            crate_name, field
                        ))?;
                    }
                }

                fn_body.push_parsed("Ok(())")?;
                Ok(())
            })?;

        Ok(())
    }

    pub fn generate_binary_parse(self, generator: &mut Generator) -> Result<()> {
        let crate_name = &self.attributes.crate_name;

        generator
            .impl_for(format!("{}::parser::BinaryParse", crate_name))
            .modify_generic_constraints(|generics, where_constraints| {
                for g in generics.iter_generics() {
                    where_constraints
                        .push_constraint(g, format!("{}::parser::BinaryParse", crate_name))
                        .unwrap();
                }
                Ok(())
            })?
            .generate_fn("binary_parse")
            .with_arg(
                "parser",
                format!("&mut {}::parser::BinaryParser", crate_name),
            )
            .with_return_type(format!("{}::error::Result<Self>", crate_name))
            .body(|fn_body| {
                // Ok(Self {
                fn_body.push_parsed("core::result::Result::Ok")?;
                fn_body.group(Delimiter::Parenthesis, |ok_group| {
                    ok_group.ident_str("Self");
                    ok_group.group(Delimiter::Brace, |struct_body| {
                        // Fields
                        // {
                        //      a: binja::parser::binary_parse(parser)?,
                        //      b: binja::parser::binary_parse(parser)?,
                        //      ...
                        // }
                        if let Some(fields) = self.fields.as_ref() {
                            for field in fields.names() {
                                // TODO: handle attributes like skip, default, etc.
                                // let _attributes = field
                                //     .attributes()
                                //     .get_attribute::<FieldAttributes>()?
                                //     .unwrap_or_default();

                                struct_body.push_parsed(format!(
                                    "{1}: {}::parser::BinaryParse::binary_parse(parser)?,",
                                    crate_name, field
                                ))?;
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
                Ok(())
            })?;
        Ok(())
    }
}
