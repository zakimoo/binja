pub mod bin_parse;
mod parser_helper;
pub mod serde_impl;

use parser_helper::ByteOrderConvertible;

use crate::config::{Config, ContainerLengthStrategy, EndiannessStrategy};
use crate::error::{Error, Result};

pub struct BinaryParser<'de> {
    input: &'de [u8],
    // Configuration for serialization (e.g., endianness, optional strategy, etc.)
    config: Config,
}

impl<'de> BinaryParser<'de> {
    pub fn new(input: &'de [u8], config: Config) -> Self {
        Self { input, config }
    }
}

impl<'de> BinaryParser<'de> {
    fn parse_bool(&mut self) -> Result<bool> {
        self.check_length::<u8>()?;

        let value = if self.input[0] == 0 {
            false
        } else if self.input[0] == 1 {
            true
        } else {
            return Err(Error::InvalidBoolValue(self.input[0]));
        };

        self.input = &self.input[1..];
        Ok(value)
    }

    fn parse_number<T>(&mut self) -> Result<T>
    where
        T: ByteOrderConvertible,
    {
        let size = self.check_length::<T>()?;

        let bytes = &self.input[..size];

        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => T::from_le_bytes(bytes),
            EndiannessStrategy::Big => T::from_be_bytes(bytes),
        };

        self.input = &self.input[size..];
        Ok(value)
    }

    pub fn parse_char(&mut self) -> Result<char> {
        if self.input.is_empty() {
            return Err(Error::NoEnoughData {
                expected: 1,
                available: 0,
            });
        }

        let result = std::str::from_utf8(self.input)
            .map_err(|_| Error::InvalidUtf8 {
                value: self.input.to_vec(),
            })?
            .chars()
            .next()
            .ok_or(Error::InvalidUtf8 {
                value: self.input.to_vec(),
            })?;

        let char_len = result.len_utf8();

        self.input = &self.input[char_len..];
        Ok(result)
    }

    pub fn parse_string(&mut self) -> Result<&'de str> {
        let len = self.parse_container_size()?;

        if self.input.len() < len {
            return Err(Error::NoEnoughData {
                expected: len,
                available: self.input.len(),
            });
        }

        let value = std::str::from_utf8(&self.input[..len])
            .ok()
            .ok_or(Error::InvalidUtf8 {
                value: self.input[..len].to_vec(),
            })?;

        self.input = &self.input[len..];
        Ok(value)
    }

    pub fn parse_container_size(&mut self) -> Result<usize> {
        let size = match self.config.container_length_strategy {
            ContainerLengthStrategy::OneByte => self.parse_number::<u8>()? as usize,
            ContainerLengthStrategy::TwoBytes => self.parse_number::<u16>()? as usize,
            ContainerLengthStrategy::FourBytes => self.parse_number::<u32>()? as usize,
            ContainerLengthStrategy::EightBytes => self.parse_number::<u64>()? as usize,
            ContainerLengthStrategy::SixteenBytes => self.parse_number::<u128>()? as usize,
        };

        Ok(size)
    }

    fn check_length<T>(&self) -> Result<usize> {
        let size = std::mem::size_of::<T>();
        let len = self.input.len();
        if len < size {
            return Err(Error::NoEnoughData {
                expected: size,
                available: len,
            });
        }

        Ok(size)
    }
}
