mod bin_parse;
#[cfg(feature = "serde")]
mod serde_impl;

pub use bin_parse::{BinaryParse, binary_parse};

use bytes::Buf;

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
    pub fn bool(&mut self) -> Result<bool> {
        match self.input.try_get_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(Error::InvalidBoolValue(x)),
        }
    }

    pub fn i8(&mut self) -> Result<i8> {
        Ok(self.input.try_get_i8()?)
    }

    pub fn i16(&mut self) -> Result<i16> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_i16_le()?,
            EndiannessStrategy::Big => self.input.try_get_i16()?,
        };
        Ok(value)
    }

    pub fn i32(&mut self) -> Result<i32> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_i32_le()?,
            EndiannessStrategy::Big => self.input.try_get_i32()?,
        };
        Ok(value)
    }

    pub fn i64(&mut self) -> Result<i64> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_i64_le()?,
            EndiannessStrategy::Big => self.input.try_get_i64()?,
        };
        Ok(value)
    }

    pub fn i128(&mut self) -> Result<i128> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_i128_le()?,
            EndiannessStrategy::Big => self.input.try_get_i128()?,
        };
        Ok(value)
    }

    fn u8(&mut self) -> Result<u8> {
        Ok(self.input.try_get_u8()?)
    }

    pub fn u16(&mut self) -> Result<u16> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_u16_le()?,
            EndiannessStrategy::Big => self.input.try_get_u16()?,
        };
        Ok(value)
    }

    pub fn u32(&mut self) -> Result<u32> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_u32_le()?,
            EndiannessStrategy::Big => self.input.try_get_u32()?,
        };
        Ok(value)
    }

    pub fn u64(&mut self) -> Result<u64> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_u64_le()?,
            EndiannessStrategy::Big => self.input.try_get_u64()?,
        };
        Ok(value)
    }

    pub fn u128(&mut self) -> Result<u128> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_u128_le()?,
            EndiannessStrategy::Big => self.input.try_get_u128()?,
        };
        Ok(value)
    }

    pub fn f32(&mut self) -> Result<f32> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_f32_le()?,
            EndiannessStrategy::Big => self.input.try_get_f32()?,
        };
        Ok(value)
    }

    pub fn f64(&mut self) -> Result<f64> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_f64_le()?,
            EndiannessStrategy::Big => self.input.try_get_f64()?,
        };
        Ok(value)
    }

    pub fn char(&mut self) -> Result<char> {
        if self.input.is_empty() {
            return Err(Error::NoEnoughData {
                expected: 1,
                available: 0,
            });
        }

        let len = if self.input.len() < 4 {
            self.input.len()
        } else {
            4
        };

        let result = std::str::from_utf8(&self.input[..len])
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

    pub fn string(&mut self) -> Result<&'de str> {
        let len = self.container_size()?;

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

    pub fn bytes(&mut self, size: usize) -> Result<&'de [u8]> {
        if self.input.len() < size {
            return Err(Error::NoEnoughData {
                expected: size,
                available: self.input.len(),
            });
        }

        let value = &self.input[..size];

        self.input = &self.input[size..];
        Ok(value)
    }

    pub fn container_size(&mut self) -> Result<usize> {
        let size = match self.config.container_length_strategy {
            ContainerLengthStrategy::OneByte => self.u8()? as usize,
            ContainerLengthStrategy::TwoBytes => self.u16()? as usize,
            ContainerLengthStrategy::FourBytes => self.u32()? as usize,
            ContainerLengthStrategy::EightBytes => self.u64()? as usize,
            ContainerLengthStrategy::SixteenBytes => self.u128()? as usize,
        };

        Ok(size)
    }
}
