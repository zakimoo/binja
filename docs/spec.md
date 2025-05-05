# Serialization Specification

default config:

- endianness_strategy: little
- optional_strategy: tagged
- container_length_strategy: 4 bytes
- limit: none

## Basic Types

- Boolean serialized as a single byte: 0x00 for false, 0x01 for true.
- Integer are serialized with fixed size:
  - 1 byte for i8 and u8
  - 2 bytes for i16 and u16
  - 4 bytes for i32 and u32
  - 8 bytes for i64 and u64
  - 16 bytes for i128 and u128
- Floating point numbers are serialized with fixed size:
  - 4 bytes for f32
  - 8 bytes for f64
- char are serialized as 4 bytes
- String are serialized as container
- unit and unit struct are not serialized
- unit variant only the variant index is serialized
- newtype struct are serialized as the inner type
- container;
  - length is serialized first depending on config container_length_strategy
  - then the elements are serialized
- tuple only the elements are serialized
  - length is not serialized
- tuple struct only the elements are serialized
  - length is not serialized
  - name is not serialized
- tuple variant variant index and elements are serialized:
  - length is not serialized
  - name is not serialized
  - variant is not serialized
- map:
  - length is serialized first depending on config container_length_strategy
  - then the key and value are serialized
- struct only filds are serialized
  - length is not serialized
  - name is not serialized
  - field names are not serialized
- struct variant variant index and fields are serialized:
  - length is not serialized
  - name is not serialized
  - field names are not serialized
