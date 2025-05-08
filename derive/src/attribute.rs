use virtue::prelude::*;
use virtue::utils::{ParsedAttribute, parse_tagged_attribute};

#[derive(Debug)]
pub struct ContainerAttributes {
    pub crate_name: String,
    pub use_serde: Option<bool>,
    pub repr: Option<String>,
}

impl Default for ContainerAttributes {
    fn default() -> Self {
        Self {
            crate_name: "::binja".to_string(),
            use_serde: None,
            repr: None,
        }
    }
}

impl FromAttribute for ContainerAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let mut result = Self::default();

        let attributes = match parse_tagged_attribute(group, "repr")? {
            Some(body) => body,
            None => return Ok(None),
        };
        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(ident) => {
                    result.repr = Some(ident.to_string());
                }
                ParsedAttribute::Property(key, _) => {
                    return Err(Error::custom_at("Unknown attribute", key.span()));
                }
                _ => {}
            }
        }

        let attributes = match parse_tagged_attribute(group, "binja")? {
            Some(body) => body,
            None => return Ok(None),
        };
        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(ident) if ident.to_string() == "serde" => {
                    result.use_serde = Some(true);
                }
                ParsedAttribute::Tag(ident) => {
                    return Err(Error::custom_at("Unknown attribute", ident.span()));
                }
                ParsedAttribute::Property(key, _) => {
                    return Err(Error::custom_at("Unknown attribute", key.span()));
                }
                _ => {}
            }
        }

        Ok(Some(result))
    }
}

#[derive(Default)]
pub struct FieldAttributes {
    pub skip: bool,
}

impl FromAttribute for FieldAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "binja")? {
            Some(body) => body,
            None => return Ok(None),
        };
        let mut result = Self::default();
        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) if i.to_string() == "skip" => {
                    result.skip = true;
                }
                ParsedAttribute::Tag(i) => {
                    return Err(Error::custom_at("Unknown field attribute", i.span()));
                }
                ParsedAttribute::Property(key, _) => {
                    return Err(Error::custom_at("Unknown field attribute", key.span()));
                }
                _ => {}
            }
        }
        Ok(Some(result))
    }
}
