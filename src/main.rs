use binary_plz::{
    config::{Config, EndiannessStrategy, OptionalStrategy},
    error::Result,
    from_bytes_with_config,
};
use serde::Deserialize;
pub fn from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let config = Config {
        endianness_strategy: EndiannessStrategy::Big,
        optional_strategy: OptionalStrategy::Tagged,
        ..Default::default()
    };

    from_bytes_with_config(bytes, config)
}
fn main() {
    let j = vec![b'a'];
    let expected: char = 'a';

    match from_bytes::<char>(&j) {
        Ok(v) => assert_eq!(expected, v),
        Err(e) => panic!("Failed to deserialize: {:?}", e),
    }
}
