use super::BinaryParser;
use crate::{config::OptionalStrategy, error::Result};
pub trait BinaryParse {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self>
    where
        Self: Sized;

    fn binary_parse_mut(&mut self, parser: &mut BinaryParser) -> Result<()>
    where
        Self: Sized,
    {
        *self = Self::binary_parse(parser)?;
        Ok(())
    }
}

impl BinaryParse for () {
    fn binary_parse(_parser: &mut BinaryParser) -> Result<Self> {
        Ok(())
    }
}

impl BinaryParse for bool {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_bool()
    }
}

impl BinaryParse for i8 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_i8()
    }
}

impl BinaryParse for i16 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_i16()
    }
}

impl BinaryParse for i32 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_i32()
    }
}

impl BinaryParse for i64 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_i64()
    }
}

impl BinaryParse for i128 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_i128()
    }
}

impl BinaryParse for u8 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_u8()
    }
}

impl BinaryParse for u16 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_u16()
    }
}

impl BinaryParse for u32 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_u32()
    }
}

impl BinaryParse for u64 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_u64()
    }
}
impl BinaryParse for u128 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_u128()
    }
}
impl BinaryParse for f32 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_f32()
    }
}

impl BinaryParse for f64 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_f64()
    }
}

impl BinaryParse for char {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.parse_char()
    }
}

impl BinaryParse for String {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        let str = parser.parse_string()?;
        Ok(str.to_string())
    }
}

impl<T> BinaryParse for Option<T>
where
    T: BinaryParse,
{
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self>
    where
        Self: Sized,
    {
        if matches!(parser.config.optional_strategy, OptionalStrategy::Tagged)
            && !parser.parse_bool()?
        {
            return Ok(None);
        }

        T::binary_parse(parser).map(Some)
    }
}

impl<T> BinaryParse for Vec<T>
where
    T: BinaryParse,
{
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self>
    where
        Self: Sized,
    {
        let len = parser.parse_container_size()?;

        let mut vec = Vec::with_capacity(len as usize);

        for _ in 0..len {
            vec.push(T::binary_parse(parser)?);
        }

        Ok(vec)
    }
}
