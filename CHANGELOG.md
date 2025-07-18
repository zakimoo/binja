# Changelog

All notable changes to this project will be documented in this file.

## [0.3.2] - 2025-06-30

### 🐛 Bug Fixes

- Enhance error formatting for better clarity and detail

## [0.3.1] - 2025-06-04

### 🐛 Bug Fixes

- Handle trait bounds correctly

### ⚙️ Miscellaneous Tasks

- Release 0.3.1

## [binja_derive-v0.1.1] - 2025-06-04

### 🐛 Bug Fixes

- Handle trait bounds correctly

### ⚙️ Miscellaneous Tasks

- Release binja_derive version 0.1.1

## [0.3.0] - 2025-05-29

### 🚀 Features

- Add is_empty method to BinaryParser and BinarySerializer

### 🐛 Bug Fixes

- Enhance BinaryParse for arrays and update tests

### ⚙️ Miscellaneous Tasks

- Release 0.3.0

## [0.2.0] - 2025-05-27

### 🚀 Features

- Implement IntoIterator for container

### 🚜 Refactor

- Serde impl serializer and deserializer

### ⚙️ Miscellaneous Tasks

- Reorder cargo_clippy hook in pre-commit configuration
- Release 0.2.0

## [0.1.0] - 2025-05-26

### 🚀 Features

- Serde compatible serializer
- Implement binary serialization
- Implement binary parsing and serialization for unit type
- Add mutable binary parsing method to BinaryParse trait
- Implement BinarySerialize and BinaryParse for OtherStruct and TestStruct
- Add binary parsing and serialization methods for arrays
- Implement binary serialization and parsing for structs and enums using the Binja derive macros
- Enhance binary serialization and parsing by integrating new attributes for structs and enums
- Implement binary serialization and parsing for enums and structs with updated attribute handling
- Implement binary serialization and parsing for collections and tuples
- Add Cargo.lock for dependency management and update parsing functions for improved clarity
- Implement skip functionality in FieldAttributes and update serialization logic
- Add bit and bit_mask macros for bit manipulation
- Add BinarySerialize derive to enum E for serialization support
- Add bit field serialization support with overflow handling
- Enhance serialization by consolidating field serialization logic for structs and enums
- Add validation logic for field attributes to enforce constraints
- Add SeparateBitField2 struct and update TupleStruct to use it
- Add VariantAttributes struct and integrate it into enum serialization and parsing
- Enhance binary parsing for fields with bit attributes, supporting bitwise operations and default handling
- Add optional serde support for serialization and deserialization
- Add validation for bit field types and introduce is_type_bool function
- Enhance serialization for structs and enums with improved field handling
- Add Unit struct and update expected serialization format in tests
- Add boolean handling in serialization and parsing for fields
- Implement optional serialization strategy for BinarySerializer
- Enhance optional parsing strategy for BinaryParse trait
- Extend tuple serialization and parsing support for additional elements
- Add FixedSizeContainer for binary serialization and parsing
- Add example for binary serialization and deserialization with TestStruct

### 🐛 Bug Fixes

- Handle input length in string parsing to prevent out-of-bounds access
- Update overflow check in bit field serialization and adjust example values
- Add parentheses to bit and bit_mask macros for clarity in bitwise operations
- Update validation logic for bits attribute to enforce range between 1 and 128
- Update SeparateBitField2 to use tuple struct syntax and remove unused serialization code
- Add missing fields to TestStruct for proper serialization and parsing
- Type in example struct.rs

### 🚜 Refactor

- Parser and parser_helper
- Streamline parsing methods and remove parser_helper
- Rename project from binary_plz to binja and update related files
- Remove unused imports and clean up struct definitions
- Remove unused main and delete_me files to clean up the project structure
- Update project name in README and remove outdated specification document
- Rename binary_plz to binja and update related methods for consistency
- Update README and serializer tests to use new binary serialization methods
- Update struct serialization functions to accept attributes by reference
- Simplify field serialization logic in generate_serialize_named_fields and generate_serialize_unnamed_fields
- Streamline serialization and parsing logic for named and unnamed fields
- Rename serialization and parsing functions for consistency
- Rename enum for clarity and add untagged variant example
- Change closure trait bound from FnMut to Fn for improved clarity
- Unify field parsing logic by consolidating gen_par_named_fields and gen_par_unnamed_fields into gen_par_fields
- Update serialization and parsing functions to return syn::Result for better error handling
- Remove unused check_length calls in deserialization implementations
- Simplify parsing code generation for structs and enums
- Streamline BinarySerialize usage and remove redundant imports
- Unify BinaryParse usage and remove redundant module

### 📚 Documentation

- Add TODO section to README for future improvements

### ⚙️ Miscellaneous Tasks

- Update criterion version to 0.6.0 in Cargo.toml and Cargo.lock
- Tooling conifg
- Add Changelog
- Release 0.1.0

<!-- generated by git-cliff -->
