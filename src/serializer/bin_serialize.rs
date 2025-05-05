use super::BinarySerializer;
use crate::error::Result;

pub trait BinarySerialize {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()>;
}

impl BinarySerialize for bool {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_bool(*self)
    }
}

impl BinarySerialize for i8 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_i8(*self)
    }
}
impl BinarySerialize for i16 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_i16(*self)
    }
}
impl BinarySerialize for i32 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_i32(*self)
    }
}
impl BinarySerialize for i64 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_i64(*self)
    }
}
impl BinarySerialize for i128 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_i128(*self)
    }
}
impl BinarySerialize for u8 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_u8(*self)
    }
}
impl BinarySerialize for u16 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_u16(*self)
    }
}
impl BinarySerialize for u32 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_u32(*self)
    }
}

impl BinarySerialize for u64 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_u64(*self)
    }
}

impl BinarySerialize for u128 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_u128(*self)
    }
}

impl BinarySerialize for f32 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_f32(*self)
    }
}

impl BinarySerialize for f64 {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_f64(*self)
    }
}

impl BinarySerialize for char {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_u32(*self as u32)
    }
}

impl BinarySerialize for String {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_str(self)
    }
}

impl BinarySerialize for &str {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_str(self)
    }
}

impl<T> BinarySerialize for Option<T>
where
    T: BinarySerialize,
{
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_optional(self)
    }
}

impl<T> BinarySerialize for Vec<T>
where
    T: BinarySerialize,
{
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put_container_length(self.len());

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
