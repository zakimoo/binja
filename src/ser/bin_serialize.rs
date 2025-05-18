use super::BinarySerializer;
use crate::error::Result;

#[inline(always)]
pub fn binary_serialize<T>(value: &T, serializer: &mut BinarySerializer) -> Result<()>
where
    T: BinarySerialize,
{
    value.binary_serialize(serializer)
}

pub trait BinarySerialize {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()>;
}

impl BinarySerialize for () {
    fn binary_serialize(&self, _serializer: &mut BinarySerializer) -> Result<()> {
        Ok(())
    }
}

impl BinarySerialize for bool {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.bool(*self)
    }
}

impl BinarySerialize for i8 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.i8(*self)
    }
}
impl BinarySerialize for i16 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.i16(*self)
    }
}
impl BinarySerialize for i32 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.i32(*self)
    }
}
impl BinarySerialize for i64 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.i64(*self)
    }
}
impl BinarySerialize for i128 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.i128(*self)
    }
}
impl BinarySerialize for u8 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.u8(*self)
    }
}
impl BinarySerialize for u16 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.u16(*self)
    }
}
impl BinarySerialize for u32 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.u32(*self)
    }
}

impl BinarySerialize for u64 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.u64(*self)
    }
}

impl BinarySerialize for u128 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.u128(*self)
    }
}

impl BinarySerialize for f32 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.f32(*self)
    }
}

impl BinarySerialize for f64 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.f64(*self)
    }
}

impl BinarySerialize for char {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.char(*self)
    }
}

impl BinarySerialize for String {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.str(self)
    }
}

impl BinarySerialize for &str {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.str(self)
    }
}

impl<T> BinarySerialize for Option<T>
where
    T: BinarySerialize,
{
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        match serializer.config.optional_strategy {
            crate::config::OptionalStrategy::Tagged => {
                if let Some(v) = self {
                    serializer.bool(true)?;
                    v.binary_serialize(serializer)?;
                } else {
                    serializer.bool(false)?;
                }
            }
            crate::config::OptionalStrategy::Untagged => {
                if let Some(v) = self {
                    v.binary_serialize(serializer)?;
                }
            }
        }
        Ok(())
    }
}

impl<T> BinarySerialize for Vec<T>
where
    T: BinarySerialize,
{
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.container_length(self.len());

        for item in self {
            item.binary_serialize(serializer)?;
        }

        Ok(())
    }
}

impl<T> BinarySerialize for &[T]
where
    T: BinarySerialize,
{
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        for item in *self {
            item.binary_serialize(serializer)?;
        }
        Ok(())
    }
}

impl<T, const N: usize> BinarySerialize for [T; N]
where
    T: BinarySerialize,
{
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        for item in self {
            item.binary_serialize(serializer)?;
        }
        Ok(())
    }
}

impl<K, V> BinarySerialize for std::collections::HashMap<K, V>
where
    K: BinarySerialize,
    V: BinarySerialize,
{
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.container_length(self.len());
        for (key, value) in self {
            key.binary_serialize(serializer)?;
            value.binary_serialize(serializer)?;
        }
        Ok(())
    }
}

impl<T> BinarySerialize for std::collections::HashSet<T>
where
    T: BinarySerialize,
{
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.container_length(self.len());
        for item in self {
            item.binary_serialize(serializer)?;
        }
        Ok(())
    }
}

impl<K, V> BinarySerialize for std::collections::BTreeMap<K, V>
where
    K: BinarySerialize,
    V: BinarySerialize,
{
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.container_length(self.len());
        for (key, value) in self {
            key.binary_serialize(serializer)?;
            value.binary_serialize(serializer)?;
        }
        Ok(())
    }
}

impl<T> BinarySerialize for std::collections::BTreeSet<T>
where
    T: BinarySerialize,
{
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.container_length(self.len());
        for item in self {
            item.binary_serialize(serializer)?;
        }
        Ok(())
    }
}

macro_rules! impl_binary_serialize_for_tuple {
    ($($name:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($name),+> BinarySerialize for ($($name,)+)
        where
            $($name: BinarySerialize),+
        {
            fn binary_serialize(
                &self,
                serializer: &mut BinarySerializer,
            ) -> Result<()> {
                let ($($name,)+) = self;
                $(
                    binary_serialize($name, serializer)?;
                )+
                Ok(())
            }
        }
    };
}

impl_binary_serialize_for_tuple!(T1);
impl_binary_serialize_for_tuple!(T1, T2);
impl_binary_serialize_for_tuple!(T1, T2, T3);
impl_binary_serialize_for_tuple!(T1, T2, T3, T4);
impl_binary_serialize_for_tuple!(T1, T2, T3, T4, T5);
impl_binary_serialize_for_tuple!(T1, T2, T3, T4, T5, T6);
impl_binary_serialize_for_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_binary_serialize_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_binary_serialize_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_binary_serialize_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_binary_serialize_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_binary_serialize_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_binary_serialize_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_binary_serialize_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_binary_serialize_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_binary_serialize_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
