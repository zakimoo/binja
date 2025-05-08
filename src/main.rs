use binary_plz::{
    config::Config,
    error::Result,
    from_bytes,
    parser::{BinaryParser, bin_parse::BinaryParse},
    serde_from_bytes, serde_to_bytes,
    serializer::{BinarySerializer, bin_serialize::BinarySerialize},
};
use serde::{Deserialize, Serialize};

// i want to test the to bytes function and i need to test all rust types unit struct enum unit struct optional ....
#[derive(Debug, Default, Deserialize, Serialize)]
struct OtherStruct {
    a: u8,
    b: i16,
}

impl BinarySerialize for OtherStruct {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        self.a.binary_serialize(serializer)?;
        self.b.binary_serialize(serializer)
    }
}

impl BinaryParse for OtherStruct {
    fn binary_parse(parser: &mut BinaryParser) -> Result<Self> {
        let a = u8::binary_parse(parser)?;
        let b = i16::binary_parse(parser)?;
        Ok(OtherStruct { a, b })
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
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
        self.a.binary_serialize(serializer)?;
        self.b.binary_serialize(serializer)?;
        self.c.binary_serialize(serializer)?;
        self.d.binary_serialize(serializer)?;
        self.e.binary_serialize(serializer)?;
        self.f.binary_serialize(serializer)?;
        self.h.binary_serialize(serializer)
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

    let parsed_struct: TestStruct = from_bytes(&bytes).unwrap();
    let parsed_struct_2: TestStruct = serde_from_bytes(&bytes_2).unwrap();

    println!("Parsed struct     : {:?}", parsed_struct);
    println!("Parsed struct 2   : {:?}", parsed_struct_2);
}
