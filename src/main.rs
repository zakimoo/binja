use binja::{from_bytes, serde_from_bytes, serde_to_bytes, to_bytes};
use binja_derive::{BinaryParse, BinarySerialize};
use serde::{Deserialize, Serialize};

// i want to test the to bytes function and i need to test all rust types unit struct enum unit struct optional ....
#[derive(Debug, Default, Deserialize, Serialize, BinarySerialize, BinaryParse)]
struct OtherStruct<T> {
    a: u8,
    b: i16,
    c: String,
    d: Option<T>,
}

#[derive(Debug, Default, Deserialize, Serialize, BinarySerialize, BinaryParse)]
struct TestStruct {
    a: u8,
    b: i16,
    c: String,
    d: OtherStruct<u32>,
    // e: Option<u32>,
    // f: Option<String>,
    // h: OtherStruct,
}

fn main() {
    let test_struct = TestStruct {
        a: 1,
        b: -2,
        c: "Hello".to_string(),
        d: OtherStruct {
            a: 5,
            b: -6,
            c: "World".to_string(),
            d: Some(4),
        },
        // e: Some(4),
        // f: Some("World".to_string()),
        // h: OtherStruct { a: 5, b: -6 },
    };

    let bytes = to_bytes(&test_struct).unwrap();
    let bytes_2 = serde_to_bytes(&test_struct).unwrap();

    println!("Serialized bytes  : {:?}", bytes.to_vec());
    println!("Serialized bytes 2: {:?}", bytes_2.to_vec());

    let parsed_struct: TestStruct = from_bytes(&bytes).unwrap();
    let parsed_struct_2: TestStruct = serde_from_bytes(&bytes_2).unwrap();

    println!("Parsed struct     : {:?}", parsed_struct);
    println!("Parsed struct 2   : {:?}", parsed_struct_2);
}
