use binja::{BinaryParse, BinarySerialize, to_bytes};

#[derive(BinaryParse, PartialEq, Eq, Debug)]
struct Unit;

#[derive(BinarySerialize, BinaryParse, PartialEq, Eq, Debug)]
struct Newtype(u32);
#[derive(BinarySerialize, BinaryParse, PartialEq, Eq, Debug)]
struct TupleStruct(u32, u32);

#[derive(BinarySerialize, BinaryParse, PartialEq, Eq, Debug)]
struct TestStruct {
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    char: char,
    string: String,
    option: Option<u32>,
    unit: (),
    newtype_struct: Newtype,
    seq: Vec<String>,
    tuple: (u32, u32, u8, u16, u32),
    bytes: [u8; 4],
    tuple_struct: TupleStruct,
    map: std::collections::BTreeMap<u8, String>,
}

impl Default for TestStruct {
    fn default() -> Self {
        let mut map = std::collections::BTreeMap::new();
        map.insert(1, "a".to_owned());
        map.insert(2, "b".to_owned());
        map.insert(3, "c".to_owned());
        TestStruct {
            u8: 1,
            u16: 2,
            u32: 3,
            u64: 4,
            i8: -5,
            i16: -6,
            i32: -7,
            i64: -8,
            char: 'a',
            string: "hello 😊".to_owned(),
            option: Some(11),
            unit: (),
            newtype_struct: Newtype(12),
            seq: vec!["a".to_owned(), "b".to_owned()],
            tuple: (13, 14, 15, 16, 17),
            bytes: [18, 19, 20, 21],
            tuple_struct: TupleStruct(22, 23),
            map,
        }
    }
}

fn main() {
    let my_struct = TestStruct::default();

    let expected = vec![
        0x01, // u8
        0x02, 0x00, // u16
        0x03, 0x00, 0x00, 0x00, // u32
        0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // u64
        0xFB, // i8
        0xFA, 0xFF, // i16
        0xF9, 0xFF, 0xFF, 0xFF, // i32
        0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // i64
        b'a', // char
        0x0A, 0x00, 0x00, 0x00, // string size
        b'h', b'e', b'l', b'l', b'o', b' ', 0xF0, 0x9F, 0x98, 0x8A, // string
        0x01, // option tag
        0x0B, 0x00, 0x00, 0x00, // option value
        // unit not serialized
        0x0C, 0x00, 0x00, 0x00, // newtype struct value
        // seq
        0x02, 0x00, 0x00, 0x00, // seq size
        0x01, 0x00, 0x00, 0x00, // string size
        b'a', // string
        0x01, 0x00, 0x00, 0x00, // string size
        b'b', // string
        // tuple
        0x0D, 0x00, 0x00, 0x00, // tuple value 1
        0x0E, 0x00, 0x00, 0x00, // tuple value 2
        0x0F, // tuple value 3
        0x10, 0x00, // tuple value 4
        0x11, 0x00, 0x00, 0x00, // tuple value 5
        // bytes
        0x12, 0x13, 0x14, 0x15, // tuple struct
        0x16, 0x00, 0x00, 0x00, // tuple struct value 1
        0x17, 0x00, 0x00, 0x00, // tuple struct value 2
        // map
        0x03, 0x00, 0x00, 0x00, // map size
        0x01, // key
        0x01, 0x00, 0x00, 0x00, // value string size
        b'a', // value string
        0x02, // key
        0x01, 0x00, 0x00, 0x00, // value string size
        b'b', // value string
        0x03, // key
        0x01, 0x00, 0x00, 0x00, // value string size
        b'c', // value string
    ];

    let serialized = to_bytes(&my_struct).unwrap().to_vec();

    assert_eq!(serialized, expected);
}
