use criterion::{black_box, criterion_group, criterion_main, Criterion};
use emdb_lib::words::beginnings::{init_en_in_to_n, init_un_to_u};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("init_en_in_to_n enlarge", |b| b.iter(|| init_en_in_to_n(black_box("enlarge"))));
    c.bench_function("init_un_in_u enlarge", |b| b.iter(|| init_un_to_u(black_box("enlarge"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
