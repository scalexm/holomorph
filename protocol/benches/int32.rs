use bytes::BytesMut;
use criterion::{black_box, criterion_group, criterion_main, Benchmark, Criterion};
use protocol::{Decode, Encode};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "benches",
        Benchmark::new("write_u32", move |b| {
            b.iter(|| {
                let mut buf = BytesMut::with_capacity(4);
                black_box(12_456_456u32).encode(black_box(&mut buf))
            })
        }),
    );

    c.bench(
        "benches",
        Benchmark::new("read_u32", move |b| {
            let mut buf = BytesMut::new();
            12_456_456u32.encode(&mut buf);
            b.iter(|| u32::decode(&mut (black_box(&buf[0..]))).unwrap())
        }),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
