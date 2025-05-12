use binja::{from_bytes, serde_from_bytes, serde_to_bytes, serializer::BinarySerialize, to_bytes};
use binja_derive::{BinaryParse, BinarySerialize};
use serde::{Deserialize, Serialize};

// // i want to test the to bytes function and i need to test all rust types unit struct enum unit struct optional ....
#[derive(BinarySerialize, BinaryParse)]
struct OtherStruct<T> {
    a: u8,
    b: i16,
    c: String,
    d: Option<T>,
}

#[derive(Debug, Default, Deserialize, Serialize, BinarySerialize, BinaryParse)]
struct StructNamed {
    #[binja(skip)]
    a: u8,
    b: i16,
    c: String,
    // d: OtherStruct<u32>,
    e: Option<u32>,
    // f: Option<String>,
    // h: OtherStruct,
}

#[derive(BinarySerialize, BinaryParse)]
struct StructNewtype(u8);

#[derive(BinarySerialize, BinaryParse)]
struct StructTuple(#[binja(skip)] u8, i16, String, Option<u32>);

#[derive(BinarySerialize)]
struct StructUnit;

#[derive(Serialize, Deserialize, BinarySerialize)]
#[repr(i16)]
#[binja(repr = "i8")]
enum TestEnum<T> {
    AA = -1,
    A = 10,
    B,
    C(u8) = 20,
    D {
        #[binja(skip)]
        a: u8,
        b: i16,
    },
    E(u8, i16),
    F {
        a: T,
        b: i16,
    },
}
fn main() {
    let test_struct = StructNamed {
        a: 1,
        b: -2,
        c: "Hello".to_string(),
        // d: OtherStruct {
        //     a: 5,
        //     b: -6,
        //     c: "World".to_string(),
        //     d: Some(4),
        // },
        e: Some(4),
        // f: Some("World".to_string()),
        // h: OtherStruct { a: 5, b: -6 },
    };

    let bytes = to_bytes(&test_struct).unwrap();
    let bytes_2 = serde_to_bytes(&test_struct).unwrap();

    println!("Serialized bytes  : {:?}", bytes.to_vec());
    println!("Serialized bytes 2: {:?}", bytes_2.to_vec());

    // let parsed_struct: TestStruct = from_bytes(&bytes).unwrap();
    // let parsed_struct_2: TestStruct = serde_from_bytes(&bytes_2).unwrap();

    // println!("Parsed struct     : {:?}", parsed_struct);
    // println!("Parsed struct 2   : {:?}", parsed_struct_2);
}
