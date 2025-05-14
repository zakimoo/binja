use binja::{
    from_bytes, parser::BinaryParse, serde_from_bytes, serde_to_bytes, serializer::BinarySerialize,
    to_bytes,
};
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

#[derive(BinarySerialize, BinaryParse)]
struct StructUnit;

#[derive(BinarySerialize, BinaryParse)]
#[repr(i16)]
#[binja(repr = "i8", untagged)]
enum TestEnum<T> {
    D {
        #[binja(skip)]
        a: u8,
        b: i16,
    },
    A = 10,
    AA(u8, i16) = -1,
    B = 2,
    C(u8) = 20,

    F {
        a: T,
        b: i16,
    },
}

// impl<T> binja::parser::BinaryParse for TestEnum<T>
// where
//     T: ::binja::parser::BinaryParse,
// {
//     fn binary_parse(parser: &mut ::binja::parser::BinaryParser) -> binja::error::Result<Self> {
//         let current_value: i8 = binja::parser::binary_parse(parser)?;
//         match current_value {
//             -1 => Ok(TestEnum::AA(
//                 binja::parser::binary_parse(parser)?,
//                 binja::parser::binary_parse(parser)?,
//             )),
//             10 => Ok(TestEnum::A),
//             11 => Ok(TestEnum::B),
//             20 => Ok(TestEnum::C(binja::parser::binary_parse(parser)?)),
//             21 => Ok(TestEnum::D {
//                 a: Default::default(),
//                 b: binja::parser::binary_parse(parser)?,
//             }),
//             22 => Ok(TestEnum::F {
//                 a: binja::parser::binary_parse(parser)?,
//                 b: binja::parser::binary_parse(parser)?,
//             }),
//             x => Err(binja::error::Error::InvalidVariant {
//                 expected: "-1, 10, 11, 20, 21, 22 or 23".to_string(),
//                 found: (format!("{}", x)),
//             }),
//         }
//     }
// }

// impl<T> binja::parser::BinaryParse for TestEnum<T>
// where
//     T: ::binja::parser::BinaryParse,
// {
//     fn binary_parse(parser: &mut ::binja::parser::BinaryParser) -> binja::error::Result<Self> {
//         Ok(TestEnum::AA(
//             binja::parser::binary_parse(parser)?,
//             binja::parser::binary_parse(parser)?,
//         ))
//     }
// }

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

    let test_enum = TestEnum::F { a: 5, b: -6 };

    let bytes = to_bytes(&test_struct).unwrap();
    let bytes_2 = serde_to_bytes(&test_struct).unwrap();

    println!("Serialized bytes  : {:?}", bytes.to_vec());
    println!("Serialized bytes 2: {:?}", bytes_2.to_vec());

    // let parsed_struct: TestStruct = from_bytes(&bytes).unwrap();
    // let parsed_struct_2: TestStruct = serde_from_bytes(&bytes_2).unwrap();

    // println!("Parsed struct     : {:?}", parsed_struct);
    // println!("Parsed struct 2   : {:?}", parsed_struct_2);
}
