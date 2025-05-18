mod ser;

pub mod config;
pub mod error;
pub mod par;

pub use ser::{BinarySerialize, serializer::BinarySerializer};

use crate::error::Result;
use bytes::BytesMut;
use config::Config;
use par::{BinaryParse, BinaryParser};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive")]
pub use binja_derive::{BinaryParse, BinarySerialize};

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
/// use binja::{to_bytes, BinarySerialize};
///
/// #[derive(BinarySerialize)]
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
pub fn to_bytes<T>(value: &T) -> Result<BytesMut>
where
    T: BinarySerialize,
{
    to_bytes_with_config(value, Config::default())
}

/// See [`to_bytes`].
#[cfg(feature = "serde")]
pub fn serde_to_bytes<T>(value: &T) -> Result<BytesMut>
where
    T: Serialize,
{
    serde_to_bytes_with_config(value, Config::default())
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
/// use binja::{to_bytes_with_config, BinarySerialize};
/// use binja::config::{Config, EndiannessStrategy, OptionalStrategy};
///
/// #[derive(BinarySerialize)]
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
pub fn to_bytes_with_config<T>(value: &T, config: Config) -> Result<BytesMut>
where
    T: BinarySerialize,
{
    let mut serializer = BinarySerializer::new(config);
    value.binary_serialize(&mut serializer)?;
    Ok(serializer.output())
}

/// See [`to_bytes_with_config`].
#[cfg(feature = "serde")]
pub fn serde_to_bytes_with_config<T>(value: &T, config: Config) -> Result<BytesMut>
where
    T: Serialize,
{
    let mut serializer = BinarySerializer::new(config);
    value.serialize(&mut ser)?;
    Ok(ser.output())
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
/// - `bytes`: The binary slice to deserialize. Must represent a valid serialized value of type `T`.
///
/// # Returns
/// - `Ok((T, usize))`: The deserialized value and the number of bytes read.
/// - `Err(Error)`: If deserialization fails or the input is invalid.
///
/// # Example
/// ```rust
/// use binja::{from_bytes, BinaryParse};
///
/// #[derive(BinaryParse, PartialEq, Debug)]
/// struct Example {
///     field1: u32,
///     field2: Option<u32>,
/// }
///
/// let bytes = vec![0x2A, 0x0, 0x0, 0x0, 0x1, 0x7, 0x0, 0x0, 0x0];
/// let (value, size): (Example, usize) = from_bytes(&bytes).unwrap();
/// assert_eq!(
///     value,
///     Example {
///         field1: 42,
///         field2: Some(7),
///     }
/// );
/// assert_eq!(size, 0);
/// ```
pub fn from_bytes<T>(bytes: &[u8]) -> Result<(T, usize)>
where
    T: BinaryParse,
{
    from_bytes_with_config(bytes, Config::default())
}

/// See [`from_bytes`].
#[cfg(feature = "serde")]
pub fn serde_from_bytes<'a, T>(bytes: &'a [u8]) -> Result<(T, usize)>
where
    T: Deserialize<'a>,
{
    serde_from_bytes_with_config(bytes, Config::default())
}

/// Deserializes a binary slice into a value of type `T` using a custom configuration.
///
/// # Parameters
/// - `bytes`: The binary slice to deserialize. Must represent a valid serialized value of type `T`.
/// - `config`: The `Config` specifying deserialization settings (endianness, optional strategy, etc.).
///
/// # Returns
/// - `Ok((T, usize))`: The deserialized value and the number of bytes read.
/// - `Err(Error)`: If deserialization fails or the input is invalid.
///
/// # Example
/// ```rust
/// use binja::{from_bytes_with_config, BinaryParse};
/// use binja::config::{Config, EndiannessStrategy, OptionalStrategy};
///
/// #[derive(BinaryParse, PartialEq, Debug)]
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
/// let (value, size): (Example, usize) = from_bytes_with_config(&bytes, config).unwrap();
/// assert_eq!(
///     value,
///     Example {
///         field1: 42,
///         field2: Some(7),
///     }
/// );
/// assert_eq!(size, 0);
/// ```
pub fn from_bytes_with_config<T>(bytes: &[u8], config: Config) -> Result<(T, usize)>
where
    T: BinaryParse,
{
    let mut deserializer = BinaryParser::new(bytes, config);

    let v = T::binary_parse(&mut deserializer)?;
    let size = deserializer.size();

    Ok((v, size))
}

/// See [`from_bytes_with_config`].
#[cfg(feature = "serde")]
pub fn serde_from_bytes_with_config<'a, T>(bytes: &'a [u8], config: Config) -> Result<(T, usize)>
where
    T: Deserialize<'a>,
{
    let mut deserializer = BinaryParser::new(bytes, config);

    let v = T::deserialize(&mut deserializer)?;
    let size = deserializer.size();

    Ok((v, size))
}

#[macro_export]
macro_rules! bit {
    ($bits:expr) => {
        (1 << $bits)
    };
}

#[macro_export]
macro_rules! bit_mask {
    ($bits:expr) => {
        ((1 << $bits) - 1)
    };
}
