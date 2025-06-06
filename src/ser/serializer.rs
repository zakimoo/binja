use bytes::{BufMut, BytesMut};

use crate::{
    config::{Config, ContainerLengthStrategy, EndiannessStrategy},
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

    /// Returns the current configuration of the serializer.
    pub fn config(&self) -> &crate::config::Config {
        &self.config
    }

    /// Returns a mutable reference to the output buffer.
    pub fn size(&self) -> usize {
        self.output.len()
    }

    /// Checks if the output buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.output.is_empty()
    }

    /// Checks if the serialized output exceeds the configured size limit.
    /// Returns an error if the limit is exceeded.
    pub fn check_limit(&self) -> Result<()> {
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
    pub fn container_length(&mut self, length: usize) {
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

    pub fn bool(&mut self, v: bool) -> Result<()> {
        self.output.put_u8(if v { 1 } else { 0 });
        self.check_limit()
    }

    pub fn i8(&mut self, v: i8) -> Result<()> {
        self.output.put_i8(v);
        self.check_limit()
    }

    pub fn i16(&mut self, v: i16) -> Result<()> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_i16(v),
            EndiannessStrategy::Little => self.output.put_i16_le(v),
        }
        self.check_limit()
    }

    pub fn i32(&mut self, v: i32) -> Result<()> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_i32(v),
            EndiannessStrategy::Little => self.output.put_i32_le(v),
        }
        self.check_limit()
    }

    pub fn i64(&mut self, v: i64) -> Result<()> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_i64(v),
            EndiannessStrategy::Little => self.output.put_i64_le(v),
        }
        self.check_limit()
    }

    pub fn i128(&mut self, v: i128) -> Result<()> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_i128(v),
            EndiannessStrategy::Little => self.output.put_i128_le(v),
        }
        self.check_limit()
    }

    pub fn u8(&mut self, v: u8) -> Result<()> {
        self.output.put_u8(v);
        self.check_limit()
    }

    pub fn u16(&mut self, v: u16) -> Result<()> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_u16(v),
            EndiannessStrategy::Little => self.output.put_u16_le(v),
        }
        self.check_limit()
    }

    pub fn u32(&mut self, v: u32) -> Result<()> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_u32(v),
            EndiannessStrategy::Little => self.output.put_u32_le(v),
        }
        self.check_limit()
    }

    pub fn u64(&mut self, v: u64) -> Result<()> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_u64(v),
            EndiannessStrategy::Little => self.output.put_u64_le(v),
        }
        self.check_limit()
    }

    pub fn u128(&mut self, v: u128) -> Result<()> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_u128(v),
            EndiannessStrategy::Little => self.output.put_u128_le(v),
        }
        self.check_limit()
    }

    pub fn f32(&mut self, v: f32) -> Result<()> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_f32(v),
            EndiannessStrategy::Little => self.output.put_f32_le(v),
        }
        self.check_limit()
    }

    pub fn f64(&mut self, v: f64) -> Result<()> {
        match self.config.endianness_strategy {
            EndiannessStrategy::Big => self.output.put_f64(v),
            EndiannessStrategy::Little => self.output.put_f64_le(v),
        }
        self.check_limit()
    }

    pub fn char(&mut self, v: char) -> Result<()> {
        self.output.put_slice(v.to_string().as_bytes());
        self.check_limit()
    }

    pub fn str(&mut self, v: &str) -> Result<()> {
        self.container_length(v.len());
        self.output.put_slice(v.as_bytes());
        self.check_limit()
    }

    pub fn bytes(&mut self, v: &[u8]) -> Result<()> {
        self.output.put_slice(v);
        self.check_limit()
    }
}
