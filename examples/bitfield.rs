use binja::{BinaryParse, BinarySerialize, from_bytes, to_bytes};

#[derive(Debug, BinarySerialize, BinaryParse)]
struct SeparateBitField {
    #[binja(bits = 1)]
    power: u8,

    #[binja(bits = 2)]
    mode: u8,

    #[binja(bits = 6)]
    error_code: u8,
}

#[derive(Debug, BinarySerialize, BinaryParse)]
#[allow(dead_code)]
struct TestStruct {
    power: SeparateBitField,

    #[binja(skip)]
    skip_0: u8,
    // 2 bits reserved for future use
    #[binja(bits = 5, no_overflow)]
    reserved: u8,

    #[binja(skip)]
    skip_1: u8,

    // 8 bits for temperature value (0-255)
    #[binja(bits = 8)]
    temperature: u8,

    #[binja(skip)]
    skip_2: u8,

    // 10 bits for pressure value (0-1023) - spans more than 1 byte but less than 2
    #[binja(bits = 10)]
    pressure: u16,

    #[binja(skip)]
    skip_3: u8,

    // 12 bits for altitude (0-4095) - just below 2 bytes
    #[binja(bits = 12)]
    altitude: u16,

    #[binja(skip)]
    skip_4: u8,

    // 5 bits for voltage level (0-31)
    #[binja(bits = 5)]
    voltage: u8,

    #[binja(skip)]
    skip_5: u8,

    // not bitfield, just a normal u8
    current: u8,

    #[binja(skip)]
    skip_6: u8,

    // 6 bits for checksum (0-63)
    #[binja(bits = 6, no_overflow)]
    checksum: u16,

    #[binja(skip)]
    skip_7: u8,
}

#[derive(Debug, BinarySerialize, BinaryParse)]
struct SeparateBitField2(
    #[binja(bits = 1)] u8,
    #[binja(bits = 2)] u8,
    #[binja(bits = 6)] u8,
);

#[derive(Debug, BinarySerialize, BinaryParse)]
struct TupleStruct(
    SeparateBitField2,
    #[binja(bits = 5, no_overflow)] u8,
    #[binja(bits = 8)] u8,
    #[binja(bits = 10)] u16,
    #[binja(bits = 12)] u16,
    #[binja(bits = 5)] u8,
    u8,
    #[binja(bits = 6, no_overflow)] u16,
);

fn main() {
    let test_struct = TestStruct {
        power: SeparateBitField {
            power: 1,
            mode: 2,
            error_code: 3,
        },
        skip_0: 1,
        reserved: 20,
        skip_1: 2,
        temperature: 25,
        skip_2: 3,
        pressure: 512,
        skip_3: 4,
        altitude: 2048,
        skip_4: 5,
        voltage: 15,
        skip_5: 6,
        current: 10,
        skip_6: 7,
        checksum: 63,
        skip_7: 8,
    };

    // Serialize the struct to bytes
    let struct_ser = to_bytes(&test_struct).unwrap();
    let struct_par: TestStruct = from_bytes(&struct_ser).unwrap().0;
    println!("serialized bytes: {:0x?}", struct_ser.to_vec());
    println!("parsed struct: {struct_par:?}");

    let sep = SeparateBitField2(1, 2, 3);
    let tuple_struct = TupleStruct(sep, 20, 25, 512, 2048, 15, 10, 63);

    let tuple_ser = to_bytes(&tuple_struct).unwrap();
    let tuple_par: TupleStruct = from_bytes(&tuple_ser).unwrap().0;

    println!("serialized tuple bytes: {:0x?}", tuple_ser.to_vec());
    println!("parsed tuple struct: {tuple_par:?}");

    assert_eq!(struct_ser, tuple_ser);
}
