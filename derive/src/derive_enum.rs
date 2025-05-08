use crate::attribute::{ContainerAttributes, FieldAttributes};
use virtue::prelude::*;

const TUPLE_FIELD_PREFIX: &str = "field_";

pub(crate) struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
    pub attributes: ContainerAttributes,
}

impl DeriveEnum {
    fn iter_fields(&self) -> EnumVariantIterator {
        EnumVariantIterator {
            idx: 0,
            variants: &self.variants,
        }
    }

    pub fn generate_binary_serialize(self, generator: &mut Generator) -> Result<()> {
        Ok(())
    }

    pub fn generate_binary_parse(self, generator: &mut Generator) -> Result<()> {
        Ok(())
    }
}

struct EnumVariantIterator<'a> {
    variants: &'a [EnumVariant],
    idx: usize,
}

impl<'a> Iterator for EnumVariantIterator<'a> {
    type Item = (Vec<TokenTree>, &'a EnumVariant);

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        let variant = self.variants.get(self.idx)?;
        self.idx += 1;

        let tokens = vec![TokenTree::Literal(Literal::u32_suffixed(idx as u32))];

        Some((tokens, variant))
    }
}
