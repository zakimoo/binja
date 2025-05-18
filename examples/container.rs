use binja::{
    BinaryParse, BinarySerialize,
    containers::{ContainerU8, SizelessContainer},
    from_bytes, to_bytes,
};

#[derive(Default, Debug, BinarySerialize, BinaryParse)]
struct TestStruct {
    normal_container: Vec<u8>, // will use default 4 bytes for length
    one_byte_container: ContainerU8<Vec<u8>>, // will use 1 byte for length
    sizeless_container: SizelessContainer<Vec<u8>>, // will use 0 bytes for length
}

fn main() {
    let test_struct = TestStruct {
        normal_container: vec![1, 2, 3],
        one_byte_container: ContainerU8::new(vec![4, 5, 6]),
        sizeless_container: SizelessContainer::new(vec![4, 5, 6]),
    };

    // Serialize
    let bytes = to_bytes(&test_struct).unwrap();
    println!("Serialized bytes: {:0x?}", bytes.to_vec());

    // Deserialize
    let deserialized: (TestStruct, usize) = from_bytes(&bytes).unwrap();
    println!("Deserialized struct: {:?}", deserialized);
}
