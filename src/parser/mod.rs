mod bin_parse;
mod serde_impl;

pub use bin_parse::BinaryParse;

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
    pub fn parse_bool(&mut self) -> Result<bool> {
        match self.input.try_get_u8()? {
            0 => {
                return Ok(false);
            }
            1 => {
                return Ok(true);
            }
            x => {
                return Err(Error::InvalidBoolValue(x));
            }
        }
    }

    pub fn parse_i8(&mut self) -> Result<i8> {
        Ok(self.input.try_get_i8()?)
    }

    pub fn parse_i16(&mut self) -> Result<i16> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_i16_le()?,
            EndiannessStrategy::Big => self.input.try_get_i16()?,
        };
        Ok(value)
    }

    pub fn parse_i32(&mut self) -> Result<i32> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_i32_le()?,
            EndiannessStrategy::Big => self.input.try_get_i32()?,
        };
        Ok(value)
    }

    pub fn parse_i64(&mut self) -> Result<i64> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_i64_le()?,
            EndiannessStrategy::Big => self.input.try_get_i64()?,
        };
        Ok(value)
    }

    pub fn parse_i128(&mut self) -> Result<i128> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_i128_le()?,
            EndiannessStrategy::Big => self.input.try_get_i128()?,
        };
        Ok(value)
    }

    fn parse_u8(&mut self) -> Result<u8> {
        Ok(self.input.try_get_u8()?)
    }

    pub fn parse_u16(&mut self) -> Result<u16> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_u16_le()?,
            EndiannessStrategy::Big => self.input.try_get_u16()?,
        };
        Ok(value)
    }

    pub fn parse_u32(&mut self) -> Result<u32> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_u32_le()?,
            EndiannessStrategy::Big => self.input.try_get_u32()?,
        };
        Ok(value)
    }

    pub fn parse_u64(&mut self) -> Result<u64> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_u64_le()?,
            EndiannessStrategy::Big => self.input.try_get_u64()?,
        };
        Ok(value)
    }

    pub fn parse_u128(&mut self) -> Result<u128> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_u128_le()?,
            EndiannessStrategy::Big => self.input.try_get_u128()?,
        };
        Ok(value)
    }

    pub fn parse_f32(&mut self) -> Result<f32> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_f32_le()?,
            EndiannessStrategy::Big => self.input.try_get_f32()?,
        };
        Ok(value)
    }

    pub fn parse_f64(&mut self) -> Result<f64> {
        let value = match self.config.endianness_strategy {
            EndiannessStrategy::Little => self.input.try_get_f64_le()?,
            EndiannessStrategy::Big => self.input.try_get_f64()?,
        };
        Ok(value)
    }

    pub fn parse_char(&mut self) -> Result<char> {
        if self.input.is_empty() {
            return Err(Error::NoEnoughData {
                expected: 1,
                available: 0,
            });
        }

        let result = std::str::from_utf8(&self.input[..4])
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

    pub fn parse_bytes(&mut self, size: usize) -> Result<&'de [u8]> {
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

    pub fn parse_container_size(&mut self) -> Result<usize> {
        let size = match self.config.container_length_strategy {
            ContainerLengthStrategy::OneByte => self.parse_u8()? as usize,
            ContainerLengthStrategy::TwoBytes => self.parse_u16()? as usize,
            ContainerLengthStrategy::FourBytes => self.parse_u32()? as usize,
            ContainerLengthStrategy::EightBytes => self.parse_u64()? as usize,
            ContainerLengthStrategy::SixteenBytes => self.parse_u128()? as usize,
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
