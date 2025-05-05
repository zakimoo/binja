use bytes::{BufMut, BytesMut};
use serde::{Serialize, ser};

use crate::{
    config::{Config, ContainerLengthStrategy, EndiannessStrategy, OptionalStrategy},
    error::{Error, Result},
};

#[derive(Debug, Default)]
pub struct BinarySerializer {
    // Buffer to store the serialized binary output
    output: BytesMut,
    // Configuration for serialization (e.g., endianness, optional strategy, etc.)
    config: Config,
}

impl BinarySerializer {
    /// Creates a new `BinarySerializer` with the specified configuration.
    pub fn new(config: Config) -> Self {
        Self {
            output: BytesMut::new(),
            config,
        }
    }

    /// Consumes the serializer and returns the serialized output as `BytesMut`.
    pub fn output(self) -> BytesMut {
        self.output
    }

    /// Checks if the serialized output exceeds the configured size limit.
    /// Returns an error if the limit is exceeded.
    fn check_limit(&self) -> Result<()> {
        if let Some(limit) = self.config.limit {
            if self.output.len() > limit {
                return Err(Error::LimitExceeded {
                    limit,
                    size: self.output.len(),
                });
            }
        }
        Ok(())
    }

    /// Writes the length of a container (e.g., sequence, string) to the output buffer
    /// based on the configured endianness and container length strategy.
    fn put_container_length(&mut self, length: usize) {
        match (
            self.config.endianness_strategy,
            self.config.container_length_strategy,
        ) {
            (_, ContainerLengthStrategy::OneByte) => {
                self.output.put_u8(length as u8);
            }
            (EndiannessStrategy::Big, ContainerLengthStrategy::TwoBytes) => {
                self.output.put_u16(length as u16);
            }
            (EndiannessStrategy::Little, ContainerLengthStrategy::TwoBytes) => {
                self.output.put_u16_le(length as u16);
            }
            (EndiannessStrategy::Big, ContainerLengthStrategy::FourBytes) => {
                self.output.put_u32(length as u32);
            }
            (EndiannessStrategy::Little, ContainerLengthStrategy::FourBytes) => {
                self.output.put_u32_le(length as u32);
            }
            (EndiannessStrategy::Big, ContainerLengthStrategy::EightBytes) => {
                self.output.put_u64(length as u64);
            }
            (EndiannessStrategy::Little, ContainerLengthStrategy::EightBytes) => {
                self.output.put_u64_le(length as u64);
            }
            (EndiannessStrategy::Big, ContainerLengthStrategy::SixteenBytes) => {
                self.output.put_u128(length as u128);
            }
            (EndiannessStrategy::Little, ContainerLengthStrategy::SixteenBytes) => {
                self.output.put_u128_le(length as u128);
            }
        }
    }
}

impl serde::ser::Serializer for &mut BinarySerializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn is_human_readable(&self) -> bool {
        false
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.output.put_u8(v as u8);
        self.check_limit()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.output.put_i8(v);
        self.check_limit()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_i16(v),
            EndiannessStrategy::Little => self.output.put_i16_le(v),
        };
        self.check_limit()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_i32(v),
            EndiannessStrategy::Little => self.output.put_i32_le(v),
        };
        self.check_limit()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_i64(v),
            EndiannessStrategy::Little => self.output.put_i64_le(v),
        };
        self.check_limit()
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_i128(v),
            EndiannessStrategy::Little => self.output.put_i128_le(v),
        };
        self.check_limit()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.output.put_u8(v);
        self.check_limit()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_u16(v),
            EndiannessStrategy::Little => self.output.put_u16_le(v),
        };
        self.check_limit()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_u32(v),
            EndiannessStrategy::Little => self.output.put_u32_le(v),
        };
        self.check_limit()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_u64(v),
            EndiannessStrategy::Little => self.output.put_u64_le(v),
        };
        self.check_limit()
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_u128(v),
            EndiannessStrategy::Little => self.output.put_u128_le(v),
        };
        self.check_limit()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_f32(v),
            EndiannessStrategy::Little => self.output.put_f32_le(v),
        };
        self.check_limit()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_f64(v),
            EndiannessStrategy::Little => self.output.put_f64_le(v),
        };
        self.check_limit()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.output.put_slice(v.to_string().as_bytes());
        self.check_limit()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.put_container_length(v.len());
        self.output.put_slice(v.as_bytes());
        self.check_limit()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.output.put_slice(v);
        self.check_limit()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        if self.config.optional_strategy == OptionalStrategy::Tagged {
            self.output.put_u8(0);
        }
        self.check_limit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        if self.config.optional_strategy == OptionalStrategy::Tagged {
            self.output.put_u8(1);
        }
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        let _ = name;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        let _ = variant;
        let _ = name;
        // TODO: add a warning that we only serialize the variant index as u8
        (variant_index as u8).serialize(self)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        let _ = name;
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        let _ = name;
        let _ = variant;
        // TODO: add a warning that we only serialize the variant index as u8
        self.output.put_u8(variant_index as u8);
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(len) = len {
            self.put_container_length(len);
        }
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        let _ = len;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let _ = len;
        let _ = name;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let _ = len;
        let _ = variant;
        let _ = name;
        // TODO: add a warning that we only serialize the variant index as u8
        self.output.put_u8(variant_index as u8);
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if let Some(len) = len {
            self.put_container_length(len);
        }
        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        let _ = len;
        let _ = name;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        let _ = len;
        let _ = variant;
        let _ = name;
        // TODO: add a warning that we only serialize the variant index as u8
        self.output.put_u8(variant_index as u8);
        Ok(self)
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl ser::SerializeSeq for &mut BinarySerializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<Self::Ok> {
        self.check_limit()
    }
}

// Same thing but for tuples.
impl ser::SerializeTuple for &mut BinarySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.check_limit()
    }
}

// Same thing but for tuple structs.
impl ser::SerializeTupleStruct for &mut BinarySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.check_limit()
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl ser::SerializeTupleVariant for &mut BinarySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.check_limit()
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl ser::SerializeMap for &mut BinarySerializer {
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.check_limit()
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl ser::SerializeStruct for &mut BinarySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.check_limit()
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl ser::SerializeStructVariant for &mut BinarySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let _ = key;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.check_limit()
    }
}
