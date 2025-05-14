/// Represents the configuration for the `binja` serializer and deserializer.
///
/// This struct allows users to customize strategies:
///
/// - `endianness_strategy`: Determines the byte order (endianness) for integers (see [`EndiannessStrategy`]).
/// - `optional_strategy`: Specifies how optional values (`Option<T>`) are serialized and deserialized (see [`OptionalStrategy`]).
/// - `container_size_strategy`: Defines the size type used for encoding the length of collections like arrays or vectors (see [`ContainerSizeStrategy`]).
/// - `limit`: Sets an optional limit for serialization or deserialization operations.
///
/// The `Config` struct provides a builder-like API to configure these strategies using methods like:
/// - `with_big_endian` / `with_little_endian`
/// - `with_tagged_optional` / `with_untagged_optional`
/// - `with_container_size_as` / `with_container_size_length`
/// - `with_limit` / `with_no_limit`
#[derive(Debug, Default, Clone, Copy)]
pub struct Config {
    pub endianness_strategy: EndiannessStrategy,
    pub optional_strategy: OptionalStrategy,
    pub container_length_strategy: ContainerLengthStrategy,
    pub limit: Option<usize>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    /// Makes binja serialize and deserialize integers in big-endian format
    pub fn with_big_endian(mut self) -> Self {
        self.endianness_strategy = EndiannessStrategy::Big;
        self
    }

    /// Makes binja serialize and deserialize integers in little-endian format.
    pub fn with_little_endian(mut self) -> Self {
        self.endianness_strategy = EndiannessStrategy::Little;
        self
    }

    pub fn with_untagged_optional(mut self) -> Self {
        self.optional_strategy = OptionalStrategy::Untagged;
        self
    }

    pub fn with_tagged_optional(mut self) -> Self {
        self.optional_strategy = OptionalStrategy::Tagged;
        self
    }

    pub fn with_container_size_as<T>(mut self) -> Self {
        match std::mem::size_of::<T>() {
            1 => self.container_length_strategy = ContainerLengthStrategy::OneByte,
            2 => self.container_length_strategy = ContainerLengthStrategy::TwoBytes,
            4 => self.container_length_strategy = ContainerLengthStrategy::FourBytes,
            8 => self.container_length_strategy = ContainerLengthStrategy::EightBytes,
            16 => self.container_length_strategy = ContainerLengthStrategy::SixteenBytes,
            _ => panic!("Unsupported size for container size strategy"),
        }
        self
    }

    pub fn with_container_size_length(mut self, length: usize) -> Self {
        match length {
            1 => self.container_length_strategy = ContainerLengthStrategy::OneByte,
            2 => self.container_length_strategy = ContainerLengthStrategy::TwoBytes,
            4 => self.container_length_strategy = ContainerLengthStrategy::FourBytes,
            8 => self.container_length_strategy = ContainerLengthStrategy::EightBytes,
            16 => self.container_length_strategy = ContainerLengthStrategy::SixteenBytes,
            _ => panic!("Unsupported size for container size strategy"),
        }
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_no_limit(mut self) -> Self {
        self.limit = None;
        self
    }
}

/// Represents the strategy for determining the byte order (endianness).
///
/// - `Little` (default): Uses little-endian byte order, where the least significant byte is stored first.
/// - `Big`: Uses big-endian byte order, where the most significant byte is stored first.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EndiannessStrategy {
    #[default]
    Little,
    Big,
}

/// Represents the strategy for serializing and deserializing optional values (`Option<T>`).
///
/// - `Tagged` (default): Adds an extra byte at the beginning to indicate whether the value exists:
///   - `0x00` for `None`.
///   - `0x01` for `Some`.
///   - The actual value follows if it exists (e.g., `0x05` for `Some(5)`).
/// - `Untagged`: Does not add an extra byte. The serialized output directly represents the value:
///   - No extra byte for `None`. (the deserializer will always try to deserialize a value)
///   - The value itself is serialized and deserialized directly for `Some`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum OptionalStrategy {
    #[default]
    Tagged,
    Untagged,
}

/// This strategy determines how much space is allocated for encoding the size of collections
/// like arrays or vectors during serialization and deserialization.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ContainerLengthStrategy {
    OneByte,
    TwoBytes,
    #[default]
    FourBytes,
    EightBytes,
    SixteenBytes,
}
