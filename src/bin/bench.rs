use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simple2fa;

fn criterion_benchmark(c: &mut Criterion) {
    let secret = simple2fa::generate_2fa_secret();
    c.bench_function("generate_2fa_code", |b| b.iter(||
        simple2fa::generate_2fa_code(&secret)
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);