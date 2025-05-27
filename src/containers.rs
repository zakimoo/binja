use std::ops::{Deref, DerefMut};

use crate::{BinaryParse, BinarySerialize, BinarySerializer, error::Result};

// type alias for ShortSizeContainer
pub type SizelessContainer<T> = FixedSizeContainer<T, 0>;
pub type ContainerU8<T> = FixedSizeContainer<T, 1>;
pub type ContainerU16<T> = FixedSizeContainer<T, 2>;
pub type ContainerU32<T> = FixedSizeContainer<T, 4>;
pub type ContainerU64<T> = FixedSizeContainer<T, 8>;
pub type ContainerU128<T> = FixedSizeContainer<T, 16>;

#[derive(Default, Debug)]
pub struct FixedSizeContainer<Container, const SIZE_BYTES: usize>(pub Container);

impl<Container, const SIZE: usize> FixedSizeContainer<Container, SIZE>
where
    Container: IntoIterator,
{
    pub fn new(data: Container) -> Self {
        FixedSizeContainer(data)
    }
}

impl<Container, const SIZE: usize> Deref for FixedSizeContainer<Container, SIZE>
where
    Container: IntoIterator,
{
    type Target = Container;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Container, const SIZE: usize> DerefMut for FixedSizeContainer<Container, SIZE>
where
    Container: IntoIterator,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const SIZE: usize, I> BinarySerialize for FixedSizeContainer<T, SIZE>
where
    for<'a> &'a T: IntoIterator<Item = &'a I>,
    I: BinarySerialize,
{
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        let len = (&self.0).into_iter().count();
        match SIZE {
            0 => {}
            1 => serializer.u8(len as u8)?,
            2 => serializer.u16(len as u16)?,
            4 => serializer.u32(len as u32)?,
            8 => serializer.u64(len as u64)?,
            16 => serializer.u128(len as u128)?,
            _ => panic!("Invalid size for FixedSizeContainer"),
        }

        for item in &self.0 {
            item.binary_serialize(serializer)?;
        }
        Ok(())
    }
}

impl<T, const SIZE: usize> BinaryParse for FixedSizeContainer<T, SIZE>
where
    T: Default + Extend<T::Item> + IntoIterator,
    T::Item: BinaryParse,
{
    fn binary_parse(parser: &mut crate::BinaryParser) -> Result<Self> {
        let len = match SIZE {
            0 => 0,
            1 => parser.u8()? as usize,
            2 => parser.u16()? as usize,
            4 => parser.u32()? as usize,
            8 => parser.u64()? as usize,
            16 => parser.u128()? as usize,
            _ => panic!("Invalid size for FixedSizeContainer"),
        };

        let mut container = T::default();

        if len == 0 {
            while let Ok(item) = T::Item::binary_parse(parser) {
                container.extend(std::iter::once(item));
            }
        } else {
            container.extend(
                (0..len)
                    .map(|_| T::Item::binary_parse(parser))
                    .collect::<Result<Vec<_>>>()?,
            );
        }

        Ok(FixedSizeContainer(container))
    }
}

impl<Container, const SIZE: usize> IntoIterator for FixedSizeContainer<Container, SIZE>
where
    Container: IntoIterator,
{
    type Item = Container::Item;
    type IntoIter = Container::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
