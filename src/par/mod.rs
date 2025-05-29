use parser::BinaryParser;

use crate::{
    config::OptionalStrategy,
    error::{Error, Result},
};
#[cfg(feature = "serde")]
mod serde_impl;

pub mod parser;

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
        match parser.config().optional_strategy {
            OptionalStrategy::Tagged => match parser.bool()? {
                true => Ok(Some(T::binary_parse(parser)?)),
                false => Ok(None),
            },
            OptionalStrategy::Untagged => match T::binary_parse(parser) {
                Ok(v) => Ok(Some(v)),
                Err(_) => Ok(None),
            },
        }
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

        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(T::binary_parse(parser)?);
        }

        Ok(vec)
    }
}

impl<T, const N: usize> BinaryParse for [T; N]
where
    T: BinaryParse,
{
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self>
    where
        Self: Sized,
    {
        let mut vec = Vec::with_capacity(N);
        for _ in 0..N {
            vec.push(T::binary_parse(parser)?);
        }

        let boxed_slice: Box<[T]> = vec.into_boxed_slice();
        let boxed_array: Box<[T; N]> = boxed_slice
            .try_into()
            .map_err(|_| Error::Message("Failed to convert Vec into array".into()))?;

        Ok(*boxed_array)
    }
}

impl<K, V> BinaryParse for std::collections::HashMap<K, V>
where
    K: BinaryParse + std::hash::Hash + Eq,
    V: BinaryParse,
{
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self>
    where
        Self: Sized,
    {
        let len = parser.container_size()?;

        let mut map = std::collections::HashMap::with_capacity(len);

        for _ in 0..len {
            let key = K::binary_parse(parser)?;
            let value = V::binary_parse(parser)?;
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<T> BinaryParse for std::collections::HashSet<T>
where
    T: BinaryParse + std::hash::Hash + Eq,
{
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self>
    where
        Self: Sized,
    {
        let len = parser.container_size()?;

        let mut set = std::collections::HashSet::with_capacity(len);

        for _ in 0..len {
            let value = T::binary_parse(parser)?;
            set.insert(value);
        }

        Ok(set)
    }
}

impl<K, V> BinaryParse for std::collections::BTreeMap<K, V>
where
    K: BinaryParse + std::cmp::Ord,
    V: BinaryParse,
{
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self>
    where
        Self: Sized,
    {
        let len = parser.container_size()?;

        let mut map = std::collections::BTreeMap::new();

        for _ in 0..len {
            let key = K::binary_parse(parser)?;
            let value = V::binary_parse(parser)?;
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<T> BinaryParse for std::collections::BTreeSet<T>
where
    T: BinaryParse + std::cmp::Ord,
{
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self>
    where
        Self: Sized,
    {
        let len = parser.container_size()?;

        let mut set = std::collections::BTreeSet::new();

        for _ in 0..len {
            let value = T::binary_parse(parser)?;
            set.insert(value);
        }

        Ok(set)
    }
}

macro_rules! impl_binary_parse_for_tuple {
    ($($name:ident),+) => {
        impl<$($name),+> BinaryParse for ($($name,)+)
        where
            $($name: BinaryParse,)*
        {
            fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
                Ok(($(
                    $name::binary_parse(parser)?,
                )+))
            }
        }
    };
}

impl_binary_parse_for_tuple!(T1);
impl_binary_parse_for_tuple!(T1, T2);
impl_binary_parse_for_tuple!(T1, T2, T3);
impl_binary_parse_for_tuple!(T1, T2, T3, T4);
impl_binary_parse_for_tuple!(T1, T2, T3, T4, T5);
impl_binary_parse_for_tuple!(T1, T2, T3, T4, T5, T6);
impl_binary_parse_for_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_binary_parse_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_binary_parse_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_binary_parse_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_binary_parse_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_binary_parse_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_binary_parse_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_binary_parse_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_binary_parse_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_binary_parse_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
