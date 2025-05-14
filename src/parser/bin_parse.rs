use super::BinaryParser;
use crate::{config::OptionalStrategy, error::Result};

#[inline(always)]
pub fn binary_parse<T>(parser: &mut BinaryParser) -> Result<T>
where
    T: BinaryParse,
{
    T::binary_parse(parser)
}

pub trait BinaryParse: Sized {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self>;

    fn binary_parse_mut(&mut self, parser: &mut BinaryParser) -> Result<()> {
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
        parser.bool()
    }
}

impl BinaryParse for i8 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.i8()
    }
}

impl BinaryParse for i16 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.i16()
    }
}

impl BinaryParse for i32 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.i32()
    }
}

impl BinaryParse for i64 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.i64()
    }
}

impl BinaryParse for i128 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.i128()
    }
}

impl BinaryParse for u8 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.u8()
    }
}

impl BinaryParse for u16 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.u16()
    }
}

impl BinaryParse for u32 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.u32()
    }
}

impl BinaryParse for u64 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.u64()
    }
}
impl BinaryParse for u128 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.u128()
    }
}
impl BinaryParse for f32 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.f32()
    }
}

impl BinaryParse for f64 {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.f64()
    }
}

impl BinaryParse for char {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        parser.char()
    }
}

impl BinaryParse for String {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        let str = parser.string()?;
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
        if matches!(parser.config.optional_strategy, OptionalStrategy::Tagged) && !parser.bool()? {
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
        let len = parser.container_size()?;

        let mut vec = Vec::with_capacity(len as usize);

        for _ in 0..len {
            vec.push(T::binary_parse(parser)?);
        }

        Ok(vec)
    }
}

impl<T, const N: usize> BinaryParse for [T; N]
where
    T: BinaryParse + Copy,
{
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self>
    where
        Self: Sized,
    {
        let arr = [T::binary_parse(parser)?; N];
        Ok(arr)
    }
}
