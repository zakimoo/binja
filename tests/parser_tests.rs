#[cfg(test)]
mod parser_little_endian_tagged_optional {
    use binary_plz::from_bytes;
    use serde::Deserialize;

    #[test]
    fn bool() {
        let j = vec![0x01];
        let expected = true;
        assert_eq!(expected, from_bytes::<bool>(&j).unwrap());

        let j = vec![0x00];
        let expected = false;
        assert_eq!(expected, from_bytes::<bool>(&j).unwrap());
    }

    #[test]
    fn integers() {
        let j = vec![0x01];
        let expected: i8 = 1;
        assert_eq!(expected, from_bytes::<i8>(&j).unwrap());

        let j = vec![0x01, 0x00];
        let expected: i16 = 1;
        assert_eq!(expected, from_bytes::<i16>(&j).unwrap());

        let j = vec![0x01, 0x00, 0x00, 0x00];
        let expected: i32 = 1;
        assert_eq!(expected, from_bytes::<i32>(&j).unwrap());

        let j = vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let expected: i64 = 1;
        assert_eq!(expected, from_bytes::<i64>(&j).unwrap());

        let j = vec![
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let expected: i128 = 1;
        assert_eq!(expected, from_bytes::<i128>(&j).unwrap());

        let j = vec![0x01];
        let expected: u8 = 1;
        assert_eq!(expected, from_bytes::<u8>(&j).unwrap());

        let j = vec![0x01, 0x00];
        let expected: u16 = 1;
        assert_eq!(expected, from_bytes::<u16>(&j).unwrap());

        let j = vec![0x01, 0x00, 0x00, 0x00];
        let expected: u32 = 1;
        assert_eq!(expected, from_bytes::<u32>(&j).unwrap());

        let j = vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let expected: u64 = 1;
        assert_eq!(expected, from_bytes::<u64>(&j).unwrap());

        let j = vec![
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let expected: u128 = 1;
        assert_eq!(expected, from_bytes::<u128>(&j).unwrap());
    }

    #[test]
    fn floats() {
        let j = vec![0x00, 0x00, 0x80, 0x3f];
        let expected: f32 = 1.0;
        assert_eq!(expected, from_bytes::<f32>(&j).unwrap());

        let j = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f];
        let expected: f64 = 1.0;
        assert_eq!(expected, from_bytes::<f64>(&j).unwrap());
    }

    #[test]
    fn char() {
        let j = vec![b'a'];
        let expected: char = 'a';
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn string() {
        let j = vec![0x01, 0x00, 0x00, 0x00, b'a'];
        let expected = "a".to_owned();
        assert_eq!(expected, from_bytes::<String>(&j).unwrap());
    }

    #[test]
    fn option() {
        let j = vec![0x01, 0x01];
        let expected = Some(1u8);
        assert_eq!(expected, from_bytes(&j).unwrap());

        let j = vec![0x00];
        let expected: Option<u8> = None;
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn unit() {
        let j = vec![];
        let expected = ();
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn newtype_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Newtype(u32);

        let j = vec![0x01, 0x00, 0x00, 0x00];
        let expected = Newtype(1);
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn seq() {
        let j = vec![
            0x02, 0x00, 0x00, 0x00, // seq size
            0x01, 0x00, 0x00, 0x00, // string size
            b'a', // string
            0x01, 0x00, 0x00, 0x00, // string size
            b'b', // string
        ];
        let expected = vec!["a".to_owned(), "b".to_owned()];
        assert_eq!(expected, from_bytes::<Vec<String>>(&j).unwrap());
    }

    #[test]
    fn tuple() {
        let j = vec![0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
        let expected = (1u32, 2u32);
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn tuple_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct TupleStruct(u32, u32);

        let j = vec![0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
        let expected = TupleStruct(1, 2);
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn map() {
        use std::collections::HashMap;

        let j = vec![
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
        let mut expected = HashMap::new();
        expected.insert("a".to_owned(), "1".to_owned());
        expected.insert("b".to_owned(), "2".to_owned());
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn test_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            opt: Option<u32>,
            seq: Vec<String>,
        }

        let expected = Test {
            int: 1,
            opt: Some(2),
            seq: vec!["a".to_owned(), "b".to_owned()],
        };

        let j = vec![
            0x01, 0x00, 0x00, 0x00, // int
            0x01, // tagged opt
            0x02, 0x00, 0x00, 0x00, // tagged opt value
            0x02, 0x00, 0x00, 0x00, // seq size
            0x01, 0x00, 0x00, 0x00, // string size
            b'a', // string
            0x01, 0x00, 0x00, 0x00, // string size
            b'b', // string
        ];
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn test_enum() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32, b: i32, c: f32 },
        }

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
}

#[cfg(test)]
mod parser_big_endian_untagged_optional {

    use binary_plz::{
        config::{Config, EndiannessStrategy, OptionalStrategy},
        error::Result,
        from_bytes_with_config,
    };
    use serde::Deserialize;

    pub fn from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        let config = Config {
            endianness_strategy: EndiannessStrategy::Big,
            optional_strategy: OptionalStrategy::Untagged,
            ..Default::default()
        };

        from_bytes_with_config(bytes, config)
    }

    #[test]
    fn bool() {
        let j = vec![0x01];
        let expected = true;
        assert_eq!(expected, from_bytes::<bool>(&j).unwrap());

        let j = vec![0x00];
        let expected = false;
        assert_eq!(expected, from_bytes::<bool>(&j).unwrap());
    }

    #[test]
    fn integers() {
        let j = vec![0x01];
        let expected: i8 = 1;
        assert_eq!(expected, from_bytes::<i8>(&j).unwrap());

        let j = vec![0x00, 0x01];
        let expected: i16 = 1;
        assert_eq!(expected, from_bytes::<i16>(&j).unwrap());

        let j = vec![0x00, 0x00, 0x00, 0x01];
        let expected: i32 = 1;
        assert_eq!(expected, from_bytes::<i32>(&j).unwrap());

        let j = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        let expected: i64 = 1;
        assert_eq!(expected, from_bytes::<i64>(&j).unwrap());

        let j = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01,
        ];
        let expected: i128 = 1;
        assert_eq!(expected, from_bytes::<i128>(&j).unwrap());

        let j = vec![0x01];
        let expected: u8 = 1;
        assert_eq!(expected, from_bytes::<u8>(&j).unwrap());

        let j = vec![0x00, 0x01];
        let expected: u16 = 1;
        assert_eq!(expected, from_bytes::<u16>(&j).unwrap());

        let j = vec![0x00, 0x00, 0x00, 0x01];
        let expected: u32 = 1;
        assert_eq!(expected, from_bytes::<u32>(&j).unwrap());

        let j = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        let expected: u64 = 1;
        assert_eq!(expected, from_bytes::<u64>(&j).unwrap());

        let j = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01,
        ];
        let expected: u128 = 1;
        assert_eq!(expected, from_bytes::<u128>(&j).unwrap());
    }

    #[test]
    fn floats() {
        let j = vec![0x3f, 0x80, 0x00, 0x00];
        let expected: f32 = 1.0;
        assert_eq!(expected, from_bytes::<f32>(&j).unwrap());

        let j = vec![0x3f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let expected: f64 = 1.0;
        assert_eq!(expected, from_bytes::<f64>(&j).unwrap());
    }

    #[test]
    fn char() {
        let j = vec![b'a'];
        let expected: char = 'a';
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn string() {
        let j = vec![0x00, 0x00, 0x00, 0x01, b'a'];
        let expected = "a".to_owned();
        assert_eq!(expected, from_bytes::<String>(&j).unwrap());
    }

    #[test]
    fn option() {
        let j = vec![0x01];
        let expected = Some(1u8);
        assert_eq!(expected, from_bytes(&j).unwrap());

        // NOTE!: untagged option will always try to deserialize the value
        // even if it is None

        // let j = vec![];
        // let expected: Option<u8> = None;
        // assert_eq!(expected, from_bytes::<Option<u8>>(&j).unwrap());
    }

    #[test]
    fn unit() {
        let j = vec![];
        let expected = ();
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn newtype_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Newtype(u32);

        let j = vec![0x00, 0x00, 0x00, 0x01];
        let expected = Newtype(1);
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn seq() {
        let j = vec![
            0x00, 0x00, 0x00, 0x02, // seq size
            0x00, 0x00, 0x00, 0x01, // string size
            b'a', // string
            0x00, 0x00, 0x00, 0x01, // string size
            b'b', // string
        ];
        let expected = vec!["a".to_owned(), "b".to_owned()];
        assert_eq!(expected, from_bytes::<Vec<String>>(&j).unwrap());
    }

    #[test]
    fn tuple() {
        let j = vec![0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
        let expected = (1u32, 2u32);
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn tuple_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct TupleStruct(u32, u32);

        let j = vec![0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
        let expected = TupleStruct(1, 2);
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn map() {
        use std::collections::HashMap;

        let j = vec![
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
        let mut expected = HashMap::new();
        expected.insert("a".to_owned(), "1".to_owned());
        expected.insert("b".to_owned(), "2".to_owned());
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn test_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            opt: Option<u32>,
            seq: Vec<String>,
        }

        let expected = Test {
            int: 1,
            opt: Some(2),
            seq: vec!["a".to_owned(), "b".to_owned()],
        };

        let j = vec![
            0x00, 0x00, 0x00, 0x01, // int
            // untagged opt
            0x00, 0x00, 0x00, 0x02, // opt value
            0x00, 0x00, 0x00, 0x02, // seq size
            0x00, 0x00, 0x00, 0x01, // string size
            b'a', // string
            0x00, 0x00, 0x00, 0x01, // string size
            b'b', // string
        ];
        assert_eq!(expected, from_bytes(&j).unwrap());
    }

    #[test]
    fn test_enum() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32, b: i32, c: f32 },
        }

        // Unit
        //  variant index --> 1 byte
        let j = vec![0x00];
        let expected = E::Unit;
        assert_eq!(expected, from_bytes(&j).unwrap());

        // new type
        let j = vec![
            0x01, //  variant index --> 1 byte
            0x00, 0x00, 0x00, 0x01, //  value --> 4 bytes
        ];
        let expected = E::Newtype(1);
        assert_eq!(expected, from_bytes(&j).unwrap());

        // tuple
        let j = vec![
            0x02, //  variant index --> 1 byte
            0x00, 0x00, 0x00, 0x01, // u32 filed value
            0x00, 0x00, 0x00, 0x02, // u32 filed value
        ];
        let expected = E::Tuple(1, 2);
        assert_eq!(expected, from_bytes(&j).unwrap());

        // struct
        let j = vec![
            0x03, //  variant index --> 1 byte
            0x00, 0x00, 0x00, 0x01, //  filed value  --> 4 bytes
            0xff, 0xff, 0xff, 0xfe, //  filed value  --> 4 bytes
            0x40, 0x40, 0x00, 0x00, //  filed value  --> 4 bytes
        ];
        let expected = E::Struct {
            a: 1,
            b: -2,
            c: 3.0,
        };
        assert_eq!(expected, from_bytes(&j).unwrap());
    }
}
