use serde::de::value::U8Deserializer;
use serde::de::{self, DeserializeSeed, IntoDeserializer, Visitor};
use serde::{Deserialize, Deserializer};

use crate::config::OptionalStrategy;
use crate::error::{Error, Result};

use super::BinaryParser;

impl<'de> de::Deserializer<'de> for &mut BinaryParser<'de> {
    type Error = Error;

    fn is_human_readable(&self) -> bool {
        false
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SizelessAccess::new(self))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i64()?)
    }

    fn deserialize_i128<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i128(self.parse_i128()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u64()?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(self.parse_u128()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_f32()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.parse_char()?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.input)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_byte_buf(self.input.to_vec())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.config.optional_strategy {
            OptionalStrategy::Untagged => visitor.visit_some(self),
            OptionalStrategy::Tagged => {
                let has_value = self.parse_bool()?;
                if has_value {
                    visitor.visit_some(self)
                } else {
                    visitor.visit_none()
                }
            }
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.parse_container_size()?;
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.parse_container_size()?;
        visitor.visit_map(MapAccess::new(self, len))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(StructAccess::new(self, fields))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(EnumAccess::new(self))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

struct SeqAccess<'a, 'de: 'a> {
    de: &'a mut BinaryParser<'de>,
    len: usize,
    index: usize,
}

impl<'de, 'a> SeqAccess<'a, 'de> {
    fn new(de: &'a mut BinaryParser<'de>, len: usize) -> Self {
        SeqAccess { de, len, index: 0 }
    }
}

impl<'de> serde::de::SeqAccess<'de> for SeqAccess<'_, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.index < self.len {
            self.index += 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len - self.index)
    }
}

struct StructAccess<'a, 'de: 'a> {
    de: &'a mut BinaryParser<'de>,
    fields: &'static [&'static str],
    index: usize,
}

impl<'a, 'de> StructAccess<'a, 'de> {
    fn new(de: &'a mut BinaryParser<'de>, fields: &'static [&'static str]) -> Self {
        StructAccess {
            de,
            fields,
            index: 0,
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for StructAccess<'_, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.index < self.fields.len() {
            seed.deserialize(self.fields[self.index].into_deserializer())
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        self.index += 1;
        seed.deserialize(&mut *self.de)
    }
}

struct MapAccess<'a, 'de: 'a> {
    de: &'a mut BinaryParser<'de>,
    len: usize,
    index: usize,
}

impl<'de, 'a> MapAccess<'a, 'de> {
    fn new(de: &'a mut BinaryParser<'de>, len: usize) -> Self {
        MapAccess { de, len, index: 0 }
    }
}

impl<'de> serde::de::MapAccess<'de> for MapAccess<'_, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.index < self.len {
            self.index += 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct EnumAccess<'a, 'de: 'a> {
    de: &'a mut BinaryParser<'de>,
}

impl<'de, 'a> EnumAccess<'a, 'de> {
    fn new(de: &'a mut BinaryParser<'de>) -> Self {
        EnumAccess { de }
    }
}

impl<'de> serde::de::EnumAccess<'de> for EnumAccess<'_, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let idx = self.de.parse_u8()?;
        let val = seed.deserialize::<U8Deserializer<Error>>(idx.into_deserializer())?;
        Ok((val, self))
    }

    fn variant<V>(self) -> std::result::Result<(V, Self::Variant), Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.variant_seed(std::marker::PhantomData)
    }
}

impl<'de> serde::de::VariantAccess<'de> for EnumAccess<'_, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        self.de.check_length::<T>()?;

        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_tuple(_len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_struct("", fields, visitor)
    }
}

struct SizelessAccess<'a, 'de: 'a> {
    de: &'a mut BinaryParser<'de>,
}

impl<'de, 'a> SizelessAccess<'a, 'de> {
    fn new(de: &'a mut BinaryParser<'de>) -> Self {
        SizelessAccess { de }
    }
}

impl<'de> serde::de::SeqAccess<'de> for SizelessAccess<'_, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        self.de.check_length::<T>()?;

        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}
