#[cfg(test)]
mod serializer_little_endian_tagged_optional {
    use binary_plz::to_bytes;
    use serde::Serialize;

    #[test]
    fn bool() {
        let expected = vec![0x01];
        let j = true;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x00];
        let j = false;
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn integers() {
        let expected = vec![0x01];
        let j: i8 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x01, 0x00];
        let j: i16 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x01, 0x00, 0x00, 0x00];
        let j: i32 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let j: i64 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let j: i128 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x01];
        let j: u8 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x01, 0x00];
        let j: u16 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x01, 0x00, 0x00, 0x00];
        let j: u32 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let j: u64 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let j: u128 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn floats() {
        let expected = vec![0x00, 0x00, 0x80, 0x3f];
        let j: f32 = 1.0;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f];
        let j: f64 = 1.0;
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn char() {
        let expected = vec![b'a'];
        let j: char = 'a';
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn string() {
        let expected = vec![0x01, 0x00, 0x00, 0x00, b'a'];
        let j = "a".to_owned();
        assert_eq!(expected, to_bytes::<String>(&j).unwrap());
    }

    #[test]
    fn option() {
        let expected = vec![0x01, 0x01];
        let j = Some(1u8);
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x00];
        let j: Option<u8> = None;
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn unit() {
        let expected: Vec<u8> = vec![];
        let j = ();
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn newtype_struct() {
        #[derive(Serialize, PartialEq, Debug)]
        struct Newtype(u32);

        let expected = vec![0x01, 0x00, 0x00, 0x00];
        let j = Newtype(1);
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn seq() {
        let expected = vec![
            0x02, 0x00, 0x00, 0x00, // seq size
            0x01, 0x00, 0x00, 0x00, // string size
            b'a', // string
            0x01, 0x00, 0x00, 0x00, // string size
            b'b', // string
        ];
        let j = vec!["a".to_owned(), "b".to_owned()];
        assert_eq!(expected, to_bytes::<Vec<String>>(&j).unwrap());
    }

    #[test]
    fn tuple() {
        let expected = vec![0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
        let j = (1u32, 2u32);
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn tuple_struct() {
        #[derive(Serialize, PartialEq, Debug)]
        struct TupleStruct(u32, u32);

        let expected = vec![0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
        let j = TupleStruct(1, 2);
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn map() {
        use std::collections::HashMap;

        let expected = vec![
            0x02, 0x00, 0x00, 0x00, // map size
            0x01, 0x00, 0x00, 0x00, // key string size
            b'a', // key string
            0x01, 0x00, 0x00, 0x00, // value string size
            b'1', // value string
            0x01, 0x00, 0x00, 0x00, // key string size
            b'b', // key string
            0x01, 0x00, 0x00, 0x00, // value string size
            b'2', // value string
        ];
        // NOTE!: HashMap does not guarantee the order of the keys
        // sort map by key
        let mut j = HashMap::new();
        j.insert("a".to_owned(), "1".to_owned());
        j.insert("b".to_owned(), "2".to_owned());

        assert_eq!(expected.len(), to_bytes(&j).unwrap().len());
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            opt: Option<u32>,
            seq: Vec<String>,
        }

        let j = Test {
            int: 1,
            opt: Some(2),
            seq: vec!["a".to_owned(), "b".to_owned()],
        };

        let expected = vec![
            0x01, 0x00, 0x00, 0x00, // int
            0x01, // tagged opt
            0x02, 0x00, 0x00, 0x00, // tagged opt value
            0x02, 0x00, 0x00, 0x00, // seq size
            0x01, 0x00, 0x00, 0x00, // string size
            b'a', // string
            0x01, 0x00, 0x00, 0x00, // string size
            b'b', // string
        ];
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize, PartialEq, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32, b: i32, c: f32 },
        }

        // Unit
        //  variant index --> 1 byte
        let expected = vec![0x00];
        let j = E::Unit;
        assert_eq!(expected, to_bytes(&j).unwrap());

        // new type
        //  variant index --> 1 byte
        //  value --> 4 bytes
        let expected = vec![0x01, 0x01, 0x00, 0x00, 0x00];
        let j = E::Newtype(1);
        assert_eq!(expected, to_bytes(&j).unwrap());

        // tuple
        //  variant index --> 1 byte
        //  value1 --> 4 bytes
        //  value2 --> 4 bytes
        let expected = vec![0x02, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
        let j = E::Tuple(1, 2);
        assert_eq!(expected, to_bytes(&j).unwrap());

        // struct
        let expected = vec![
            0x03, //  variant index --> 1 byte
            0x01, 0x00, 0x00, 0x00, //  filed value  --> 4 bytes
            0xfe, 0xff, 0xff, 0xff, //  filed value  --> 4 bytes
            0x00, 0x00, 0x40, 0x40, //  filed value  --> 4 bytes
        ];
        let j = E::Struct {
            a: 1,
            b: -2,
            c: 3.0,
        };
        assert_eq!(expected, to_bytes(&j).unwrap());
    }
}

#[cfg(test)]
mod serializer_big_endian_untagged_optional {

    use binary_plz::{
        config::{Config, EndiannessStrategy, OptionalStrategy},
        error::Result,
        to_bytes_with_config,
    };
    use bytes::BytesMut;
    use serde::Serialize;

    pub fn to_bytes<T>(value: &T) -> Result<BytesMut>
    where
        T: Serialize,
    {
        let config = Config {
            endianness_strategy: EndiannessStrategy::Big,
            optional_strategy: OptionalStrategy::Untagged,
            ..Default::default()
        };

        to_bytes_with_config(value, config)
    }

    #[test]
    fn bool() {
        let expected = vec![0x01];
        let j = true;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x00];
        let j = false;
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn integers() {
        let expected = vec![0x01];
        let j: i8 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x00, 0x01];
        let j: i16 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x00, 0x00, 0x00, 0x01];
        let j: i32 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        let j: i64 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01,
        ];
        let j: i128 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x01];
        let j: u8 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x00, 0x01];
        let j: u16 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x00, 0x00, 0x00, 0x01];
        let j: u32 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        let j: u64 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01,
        ];
        let j: u128 = 1;
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn floats() {
        let expected = vec![0x3f, 0x80, 0x00, 0x00];
        let j: f32 = 1.0;
        assert_eq!(expected, to_bytes(&j).unwrap());

        let expected = vec![0x3f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let j: f64 = 1.0;
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn char() {
        let expected = vec![b'a'];
        let j: char = 'a';
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn string() {
        let expected = vec![0x00, 0x00, 0x00, 0x01, b'a'];
        let j = "a".to_owned();
        assert_eq!(expected, to_bytes::<String>(&j).unwrap());
    }

    #[test]
    fn option() {
        let expected = vec![0x01];
        let j = Some(1u8);
        assert_eq!(expected, to_bytes(&j).unwrap());

        // NOTE!: untagged option will always try to deserialize the value
        // even if it is None

        // let expected = vec![];
        // let j: Option<u8> = None;
        // assert_eq!(expected, to_bytes::<Option<u8>>(&j).unwrap());
    }

    #[test]
    fn unit() {
        let expected: Vec<u8> = vec![];
        let j = ();
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn newtype_struct() {
        #[derive(Serialize, PartialEq, Debug)]
        struct Newtype(u32);

        let expected = vec![0x00, 0x00, 0x00, 0x01];
        let j = Newtype(1);
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn seq() {
        let expected = vec![
            0x00, 0x00, 0x00, 0x02, // seq size
            0x00, 0x00, 0x00, 0x01, // string size
            b'a', // string
            0x00, 0x00, 0x00, 0x01, // string size
            b'b', // string
        ];
        let j = vec!["a".to_owned(), "b".to_owned()];
        assert_eq!(expected, to_bytes::<Vec<String>>(&j).unwrap());
    }

    #[test]
    fn tuple() {
        let expected = vec![0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
        let j = (1u32, 2u32);
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn tuple_struct() {
        #[derive(Serialize, PartialEq, Debug)]
        struct TupleStruct(u32, u32);

        let expected = vec![0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
        let j = TupleStruct(1, 2);
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn map() {
        use std::collections::HashMap;

        let expected = vec![
            0x00, 0x00, 0x00, 0x02, // map size
            0x00, 0x00, 0x00, 0x01, // key string size
            b'a', // key string
            0x00, 0x00, 0x00, 0x01, // value string size
            b'1', // value string
            0x00, 0x00, 0x00, 0x01, // key string size
            b'b', // key string
            0x00, 0x00, 0x00, 0x01, // value string size
            b'2', // value string
        ];

        // NOTE!: HashMap does not guarantee the order of the keys
        let mut j = HashMap::new();
        j.insert("a".to_owned(), "1".to_owned());
        j.insert("b".to_owned(), "2".to_owned());
        assert_eq!(expected.len(), to_bytes(&j).unwrap().len());
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            opt: Option<u32>,
            seq: Vec<String>,
        }

        let j = Test {
            int: 1,
            opt: Some(2),
            seq: vec!["a".to_owned(), "b".to_owned()],
        };

        let expected = vec![
            0x00, 0x00, 0x00, 0x01, // int
            // untagged opt
            0x00, 0x00, 0x00, 0x02, // opt value
            0x00, 0x00, 0x00, 0x02, // seq size
            0x00, 0x00, 0x00, 0x01, // string size
            b'a', // string
            0x00, 0x00, 0x00, 0x01, // string size
            b'b', // string
        ];
        assert_eq!(expected, to_bytes(&j).unwrap());
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize, PartialEq, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32, b: i32, c: f32 },
        }

        // Unit
        //  variant index --> 1 byte
        let expected = vec![0x00];
        let j = E::Unit;
        assert_eq!(expected, to_bytes(&j).unwrap());

        // new type
        let expected = vec![
            0x01, //  variant index --> 1 byte
            0x00, 0x00, 0x00, 0x01, //  value --> 4 bytes
        ];
        let j = E::Newtype(1);
        assert_eq!(expected, to_bytes(&j).unwrap());

        // tuple
        let expected = vec![
            0x02, //  variant index --> 1 byte
            0x00, 0x00, 0x00, 0x01, // u32 filed value
            0x00, 0x00, 0x00, 0x02, // u32 filed value
        ];
        let j = E::Tuple(1, 2);
        assert_eq!(expected, to_bytes(&j).unwrap());

        // struct
        let expected = vec![
            0x03, //  variant index --> 1 byte
            0x00, 0x00, 0x00, 0x01, //  filed value  --> 4 bytes
            0xff, 0xff, 0xff, 0xfe, //  filed value  --> 4 bytes
            0x40, 0x40, 0x00, 0x00, //  filed value  --> 4 bytes
        ];
        let j = E::Struct {
            a: 1,
            b: -2,
            c: 3.0,
        };
        assert_eq!(expected, to_bytes(&j).unwrap());
    }
}
