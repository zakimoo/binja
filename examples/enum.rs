use binja::{BinaryParse, BinarySerialize, from_bytes};
#[derive(BinarySerialize, BinaryParse, PartialEq, Debug)]
#[binja(repr = "u8")]
enum E {
    Unit,
    Newtype(u32),
    Tuple(u32, u32),
    Struct { a: u32, b: i32, c: f32 },
}

fn main() {
    // Unit
    //  variant index --> 1 byte
    let j = vec![0x00];
    let expected = E::Unit;
    assert_eq!(expected, from_bytes(&j).unwrap());

    // new type
    //  variant index --> 1 byte
    //  value --> 4 bytes
    let j = vec![0x01, 0x01, 0x00, 0x00, 0x00];
    let expected = E::Newtype(1);
    assert_eq!(expected, from_bytes(&j).unwrap());

    // tuple
    //  variant index --> 1 byte
    //  value1 --> 4 bytes
    //  value2 --> 4 bytes
    let j = vec![0x02, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
    let expected = E::Tuple(1, 2);
    assert_eq!(expected, from_bytes(&j).unwrap());

    // struct
    let j = vec![
        0x03, //  variant index --> 1 byte
        0x01, 0x00, 0x00, 0x00, //  filed value  --> 4 bytes
        0xfe, 0xff, 0xff, 0xff, //  filed value  --> 4 bytes
        0x00, 0x00, 0x40, 0x40, //  filed value  --> 4 bytes
    ];
    let expected = E::Struct {
        a: 1,
        b: -2,
        c: 3.0,
    };
    assert_eq!(expected, from_bytes(&j).unwrap());
}
