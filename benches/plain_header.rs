use criterion::{criterion_group, criterion_main, Criterion};
use weecrypt::models::PlainHeader;

fn criterion_benchmark(c: &mut Criterion) {
    let p = PlainHeader::new([0; 12]);
    let bytes: [u8; 16] = [
        0x77, 0x65, 0x65, // "wee" file_extension
        0x01, // version: 1
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
    ];
    c.bench_function("as_bytes", |b| b.iter(|| p.as_bytes()));
    c.bench_function("from_bytes", |b| b.iter(|| PlainHeader::from_bytes(&bytes)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
