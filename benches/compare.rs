use binja::{
    error::Result,
    ser::{BinarySerialize, BinarySerializer},
    to_bytes,
};
use criterion::{Criterion, criterion_group, criterion_main};

struct TestStruct {
    a: u8,
    b: i16,
    c: String,
    d: Vec<u16>,
    e: Option<u32>,
    f: Option<String>,
}

impl Default for TestStruct {
    fn default() -> Self {
        TestStruct {
            a: 1,
            b: -2,
            c: "Hello".to_string(),
            d: vec![1; 1000000],
            e: Some(4),
            // add large string to test performance
            f: Some("World ".repeat(1000000)),
        }
    }
}

impl BinarySerialize for TestStruct {
    fn binary_serialize(&self, serializer: &mut BinarySerializer) -> Result<()> {
        self.a.binary_serialize(serializer)?;
        self.b.binary_serialize(serializer)?;
        self.c.binary_serialize(serializer)?;
        self.d.binary_serialize(serializer)?;
        self.e.binary_serialize(serializer)?;
        self.f.binary_serialize(serializer)
    }
}

pub fn benchmark_serialization(c: &mut Criterion) {
    let test_struct = TestStruct::default();

    c.bench_function("to_bytes", |b| b.iter(|| to_bytes(&test_struct).unwrap()));
}

criterion_group!(benches, benchmark_serialization);
criterion_main!(benches);
