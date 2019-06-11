use bytes::BytesMut;
use criterion::{black_box, criterion_group, criterion_main, Benchmark, Criterion};
use protocol::{Decode, Encode, Var};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "benches",
        Benchmark::new("write_var_u64", move |b| {
            b.iter(|| {
                let mut buf = BytesMut::with_capacity(10);
                Var(black_box(12_456_456_456_465_464u64)).encode(black_box(&mut buf))
            })
        }),
    );

    c.bench(
        "benches",
        Benchmark::new("write_var_u32", move |b| {
            b.iter(|| {
                let mut buf = BytesMut::with_capacity(10);
                Var(black_box(3_000_000_000u32)).encode(black_box(&mut buf))
            })
        }),
    );

    c.bench(
        "benches",
        Benchmark::new("read_var_u64", move |b| {
            let mut buf = BytesMut::new();
            Var(12_456_456_456_465_464u64).encode(&mut buf);
            b.iter(|| Var::<u64>::decode(&mut (black_box(&buf[0..]))).unwrap())
        }),
    );

    c.bench(
        "benches",
        Benchmark::new("read_var_u32", move |b| {
            let mut buf = BytesMut::new();
            Var(3_000_000_000u32).encode(&mut buf);
            b.iter(|| Var::<u32>::decode(&mut (black_box(&buf[0..]))).unwrap())
        }),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
