use binja::{BinaryParse, BinarySerialize, to_bytes};

#[derive(Debug, BinarySerialize, BinaryParse)]
struct TestStruct {
    // 1 bit for power status (0 = off, 1 = on)
    #[binja(bits = 1)]
    power: u8,

    // 2 bits for mode (00 = standby, 01 = active, etc.)
    #[binja(bits = 2)]
    mode: u8,

    // 3 bits for error codes (0-7)
    #[binja(bits = 3)]
    error_code: u8,

    // 2 bits reserved for future use
    #[binja(bits = 5, no_overflow)]
    reserved: u8,

    // 8 bits for temperature value (0-255)
    #[binja(bits = 8)]
    temperature: u8,

    // 10 bits for pressure value (0-1023) - spans more than 1 byte but less than 2
    #[binja(bits = 10)]
    pressure: u16,

    // 12 bits for altitude (0-4095) - just below 2 bytes
    #[binja(bits = 12)]
    altitude: u16,

    // 5 bits for voltage level (0-31)
    #[binja(bits = 5)]
    voltage: u8,

    // not bitfield, just a normal u8
    current: u8,

    // 6 bits for checksum (0-63)
    #[binja(bits = 6, no_overflow)]
    checksum: u16,
}

#[derive(Debug, BinarySerialize, BinaryParse)]
struct TupleStruct(
    #[binja(bits = 1)] u8,
    #[binja(bits = 2)] u8,
    #[binja(bits = 3)] u8,
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
        power: 1,
        mode: 2,
        error_code: 3,
        reserved: 20,
        temperature: 25,
        pressure: 512,
        altitude: 2048,
        voltage: 15,
        current: 10,
        checksum: 63,
    };

    let tuple_struct = TupleStruct(1, 2, 3, 20, 25, 512, 2048, 15, 10, 63);

    // Serialize the struct to bytes
    let serialized_bytes_1 = to_bytes(&test_struct).unwrap();
    // 00011101
    // 11001101
    // 00000000
    // 00010000
    // 00000000
    // 00011111
    // 00001010
    // 00111111
    for byte in &serialized_bytes_1 {
        println!("{:08b} ", byte);
    }

    let serialized_bytes_2 = to_bytes(&tuple_struct).unwrap();
    // 00011101
    // 11001101
    // 00000000
    // 00010000
    // 00000000
    // 00011111
    // 00001010
    // 00111111
    for byte in &serialized_bytes_2 {
        println!("{:08b} ", byte);
    }

    assert_eq!(serialized_bytes_1, serialized_bytes_2);

    // // Deserialize the bytes back to the struct
    // let deserialized_struct: TestStruct = from_bytes(&serialized_bytes).unwrap();
    // println!("Deserialized struct: {:?}", deserialized_struct);
}
