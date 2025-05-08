use binary_plz::{
    config::{Config, ContainerLengthStrategy, EndiannessStrategy, OptionalStrategy},
    error::Result,
    parser::{BinaryParser, bin_parse::BinaryParse},
    serde_from_bytes_with_config, serde_to_bytes,
    serializer::{BinarySerializer, bin_serialize::BinarySerialize},
};
use serde::{Deserialize, Serialize};

// i want to test the to bytes function and i need to test all rust types unit struct enum unit struct optional ....
#[derive(Debug, Deserialize, Serialize)]
struct OtherStruct {
    a: u8,
    b: i16,
}

impl BinarySerialize for OtherStruct {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put(&self.a)?;
        serializer.put(&self.b)
    }
}

impl BinaryParse for OtherStruct {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        let a = u8::binary_parse(parser)?;
        let b = i16::binary_parse(parser)?;
        Ok(OtherStruct { a, b })
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestStruct {
    a: u8,
    b: i16,
    c: String,
    d: Vec<u8>,
    e: Option<u32>,
    f: Option<String>,
    h: OtherStruct,
}

impl BinarySerialize for TestStruct {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        serializer.put(&self.a)?;
        serializer.put(&self.b)?;
        serializer.put(&self.c)?;
        serializer.put(&self.d)?;
        serializer.put(&self.e)?;
        serializer.put(&self.f)?;
        serializer.put(&self.h)
    }
}

impl BinaryParse for TestStruct {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        let a = u8::binary_parse(parser)?;
        let b = i16::binary_parse(parser)?;
        let c = String::binary_parse(parser)?;
        let d = Vec::<u8>::binary_parse(parser)?;
        let e = Option::<u32>::binary_parse(parser)?;
        let f = Option::<String>::binary_parse(parser)?;
        let h = OtherStruct::binary_parse(parser)?;
        Ok(TestStruct {
            a,
            b,
            c,
            d,
            e,
            f,
            h,
        })
    }
}

fn main() {
    let test_struct = TestStruct {
        a: 1,
        b: -2,
        c: "Hello".to_string(),
        d: vec![1, 2, 3],
        e: Some(4),
        f: Some("World".to_string()),
        h: OtherStruct { a: 5, b: -6 },
    };

    let mut serializer = BinarySerializer::new(Config::default());
    test_struct.binary_serialize(&mut serializer).unwrap();
    let bytes = serializer.output();

    let bytes_2 = serde_to_bytes(&test_struct).unwrap();

    println!("Serialized bytes  : {:?}", bytes.to_vec());
    println!("Serialized bytes 2: {:?}", bytes_2.to_vec());
}
