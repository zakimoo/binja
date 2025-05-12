#[derive(BinarySerialize, BinaryParse)]
struct OtherStruct<T> {
    #[binja(skip)]
    a: u8,
    b: i16,
    c: String,
    d: Option<T>,
}

impl<T> ::binja::serializer::BinarySerialize for OtherStruct<T>
where
    T: ::binja::serializer::BinarySerialize,
{
    fn binary_serialize(
        &self,
        serializer: &mut ::binja::serializer::BinarySerializer,
    ) -> ::binja::error::Result<()> {
        // skip
        //    ::binja::serializer::binary_serialize(&self.a, serializer)?;
        ::binja::serializer::binary_serialize(&self.b, serializer)?;
        ::binja::serializer::binary_serialize(&self.c, serializer)?;
        ::binja::serializer::binary_serialize(&self.d, serializer)?;
        Ok(())
    }
}

impl<T> ::binja::parser::BinaryParse for OtherStruct<T>
where
    T: ::binja::parser::BinaryParse,
{
    fn binary_parse(parser: &mut ::binja::parser::BinaryParser) -> ::binja::error::Result<Self> {
        core::result::Result::Ok(Self {
            a: ::binja::parser::BinaryParse::binary_parse(parser)?,
            b: ::binja::parser::BinaryParse::binary_parse(parser)?,
            c: ::binja::parser::BinaryParse::binary_parse(parser)?,
            d: ::binja::parser::BinaryParse::binary_parse(parser)?,
        })
    }
}

#[derive(BinarySerialize, BinaryParse)]
struct TestStruct {
    a: u8,
    b: i16,
    c: String,
    d: OtherStruct<u32>,
    e: Option<u32>,
    f: Option<String>,
    // h: OtherStruct,
}

impl ::binja::serializer::BinarySerialize for TestStruct {
    fn binary_serialize(
        &self,
        serializer: &mut ::binja::serializer::BinarySerializer,
    ) -> ::binja::error::Result<()> {
        ::binja::serializer::binary_serialize(&self.a, serializer)?;
        ::binja::serializer::binary_serialize(&self.b, serializer)?;
        ::binja::serializer::binary_serialize(&self.c, serializer)?;
        ::binja::serializer::binary_serialize(&self.d, serializer)?;
        ::binja::serializer::binary_serialize(&self.e, serializer)?;
        ::binja::serializer::binary_serialize(&self.f, serializer)?;
        Ok(())
    }
}

impl ::binja::parser::BinaryParse for TestStruct {
    fn binary_parse(parser: &mut ::binja::parser::BinaryParser) -> ::binja::error::Result<Self> {
        core::result::Result::Ok(Self {
            a: ::binja::parser::BinaryParse::binary_parse(parser)?,
            b: ::binja::parser::BinaryParse::binary_parse(parser)?,
            c: ::binja::parser::BinaryParse::binary_parse(parser)?,
            d: ::binja::parser::BinaryParse::binary_parse(parser)?,
            e: ::binja::parser::BinaryParse::binary_parse(parser)?,
            f: ::binja::parser::BinaryParse::binary_parse(parser)?,
        })
    }
}

#[derive(BinarySerialize)]
#[repr(u16)]
#[binja(repr = u32, untagged)]
enum TestEnum {
    AA,
    A = 10,
    B,
    C(u8) = 20,
    D { a: u8, b: i16 },
    E(u8, i16),
}

impl ::binja::serializer::BinarySerialize for TestEnum {
    fn binary_serialize(
        &self,
        serializer: &mut ::binja::serializer::BinarySerializer,
    ) -> ::binja::error::Result<()> {
        match self {
            Self::AA => {
                let value: u32 = 0;
                ::binja::serializer::binary_serialize(&value, serializer)?;
            }
            Self::A => {
                let value: u32 = 10;
                ::binja::serializer::binary_serialize(&value, serializer)?;
            }
            Self::B => {
                let value: u32 = 11;
                ::binja::serializer::binary_serialize(&value, serializer)?;
            }
            Self::C(field_0) => {
                let value: u32 = 20;
                ::binja::serializer::binary_serialize(&value, serializer)?;
                ::binja::serializer::binary_serialize(field_0, serializer)?;
            }
            Self::D { a, b } => {
                let value: u32 = 21;
                ::binja::serializer::binary_serialize(&value, serializer)?;
                ::binja::serializer::binary_serialize(a, serializer)?;
                ::binja::serializer::binary_serialize(b, serializer)?;
            }
            Self::E(field_0, field_1) => {
                let value: u32 = 22;
                ::binja::serializer::binary_serialize(&value, serializer)?;
                ::binja::serializer::binary_serialize(field_0, serializer)?;
                ::binja::serializer::binary_serialize(field_1, serializer)?;
            }
        }
        Ok(())
    }
}
