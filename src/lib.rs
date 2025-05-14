pub mod config;
pub mod error;
pub mod parser;
pub mod serializer;

use crate::error::Result;
use bytes::BytesMut;
use config::Config;
use parser::{BinaryParse, BinaryParser};
use serde::{Deserialize, Serialize};
use serializer::{BinarySerialize, BinarySerializer};

/// Serializes a given value into a binary format using the default configuration.
///
/// # Default Configuration
/// - **Optional Strategy**: Tagged (uses a single byte to indicate `Some` or `None`)
/// - **Endianness**: Little-endian
/// - **Limit**: No size limit
/// - **Container Length**: 4 bytes (used to encode the length of sequences, strings, etc.)
///
/// # Parameters
/// - `value`: A reference to the value to be serialized. The value must implement the `Serialize` trait.
///
/// # Returns
/// - `Ok(BytesMut)`: The serialized binary representation of the value.
/// - `Err(Error)`: An error if serialization fails or exceeds the configured limit.
///
/// # Example
/// ```rust
/// use binja::to_bytes;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Example {
///     field1: u32,
///     field2: Option<u32>,
/// }
///
/// let value = Example {
///     field1: 42,
///     field2: Some(7),
/// };
///
/// let serialized = to_bytes(&value).unwrap().to_vec();
/// assert_eq!(serialized, vec![0x2A, 0x0, 0x0, 0x0, 0x1, 0x7, 0x0, 0x0, 0x0]);
/// ```
pub fn serde_to_bytes<T>(value: &T) -> Result<BytesMut>
where
    T: Serialize,
{
    serde_to_bytes_with_config(value, Config::default())
}

pub fn to_bytes<T>(value: &T) -> Result<BytesMut>
where
    T: BinarySerialize,
{
    to_bytes_with_config(value, Config::default())
}

/// Serializes a given value into a binary format using a custom configuration.
///
/// # Parameters
/// - `value`: A reference to the value to be serialized. The value must implement the `Serialize` trait.
/// - `config`: A `Config` object specifying the serialization settings (e.g., endianness, optional strategy, etc.).
///
/// # Returns
/// - `Ok(BytesMut)`: The serialized binary representation of the value.
/// - `Err(Error)`: An error if serialization fails or exceeds the configured limit.
///
/// # Example
/// ```rust
/// use binja::to_bytes_with_config;
/// use binja::config::{Config, EndiannessStrategy, OptionalStrategy};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Example {
///     field1: u32,
///     field2: Option<u32>,
/// }
///
/// let config = Config {
///     endianness_strategy: EndiannessStrategy::Big,
///     optional_strategy: OptionalStrategy::Tagged,
///     ..Default::default()
/// };
///
/// let value = Example {
///     field1: 42,
///     field2: Some(7),
/// };
///
/// let serialized = to_bytes_with_config(&value, config).unwrap().to_vec();
/// assert_eq!(serialized, vec![0x00, 0x00, 0x00, 0x2A, 0x01, 0x00, 0x00, 0x00, 0x07]);
/// ```
pub fn serde_to_bytes_with_config<T>(value: &T, config: Config) -> Result<BytesMut>
where
    T: Serialize,
{
    let mut serializer = BinarySerializer::new(config);
    value.serialize(&mut serializer)?;
    Ok(serializer.output())
}

pub fn to_bytes_with_config<T>(value: &T, config: Config) -> Result<BytesMut>
where
    T: BinarySerialize,
{
    let mut serializer = BinarySerializer::new(config);
    value.binary_serialize(&mut serializer)?;
    Ok(serializer.output())
}

/// Deserializes a binary slice into a value of type `T` using the default configuration.
///
/// # Default Configuration
/// - **Optional Strategy**: Tagged (uses a single byte to indicate `Some` or `None`)
/// - **Endianness**: Little-endian
/// - **Limit**: No size limit
/// - **Container Length**: 4 bytes (used to decode the length of sequences, strings, etc.)
///
/// # Parameters
/// - `bytes`: A reference to the binary slice to be deserialized. The slice must represent a valid serialized value of type `T`.
///
/// # Returns
/// - `Ok(T)`: The deserialized value of type `T`.
/// - `Err(Error)`: An error if deserialization fails or the binary slice is invalid.
///
/// # Example
/// ```rust
/// use binja::from_bytes;
/// use serde::Deserialize;
///
/// #[derive(Deserialize, PartialEq, Debug)]
/// struct Example {
///     field1: u32,
///     field2: Option<u32>,
/// }
///
/// let bytes = vec![0x2A, 0x0, 0x0, 0x0, 0x1, 0x7, 0x0, 0x0, 0x0];
/// let value: Example = from_bytes(&bytes).unwrap();
/// assert_eq!(
///     value,
///     Example {
///         field1: 42,
///         field2: Some(7),
///     }
/// );
/// ```
pub fn serde_from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    serde_from_bytes_with_config(bytes, Config::default())
}

pub fn from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: BinaryParse,
{
    from_bytes_with_config(bytes, Config::default())
}

/// Deserializes a binary slice into a value of type `T` using a custom configuration.
///
/// # Parameters
/// - `bytes`: A reference to the binary slice to be deserialized. The slice must represent a valid serialized value of type `T`.
/// - `config`: A `Config` object specifying the deserialization settings (e.g., endianness, optional strategy, etc.).
///
/// # Returns
/// - `Ok(T)`: The deserialized value of type `T`.
/// - `Err(Error)`: An error if deserialization fails or the binary slice is invalid.
///
/// # Example
/// ```rust
/// use binja::from_bytes_with_config;
/// use binja::config::{Config, EndiannessStrategy, OptionalStrategy};
/// use serde::Deserialize;
///
/// #[derive(Deserialize, PartialEq, Debug)]
/// struct Example {
///     field1: u32,
///     field2: Option<u32>,
/// }
///
/// let config = Config {
///     endianness_strategy: EndiannessStrategy::Big,
///     optional_strategy: OptionalStrategy::Tagged,
///     ..Default::default()
/// };
///
/// let bytes = vec![0x00, 0x00, 0x00, 0x2A, 0x01, 0x00, 0x00, 0x00, 0x07];
/// let value: Example = from_bytes_with_config(&bytes, config).unwrap();
/// assert_eq!(
///     value,
///     Example {
///         field1: 42,
///         field2: Some(7),
///     }
/// );
/// ```
pub fn serde_from_bytes_with_config<'a, T>(bytes: &'a [u8], config: Config) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = BinaryParser::new(bytes, config);

    T::deserialize(&mut deserializer)
}

pub fn from_bytes_with_config<'a, T>(bytes: &'a [u8], config: Config) -> Result<T>
where
    T: BinaryParse,
{
    let mut deserializer = BinaryParser::new(bytes, config);

    T::binary_parse(&mut deserializer)
}
