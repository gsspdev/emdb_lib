use criterion::{black_box, criterion_group, criterion_main, Criterion};
use emdb_lib::word_beginnings::{init_en_in_to_n, remove_init_chars};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("init_en_in_to_n enlarge", |b| b.iter(|| init_en_in_to_n(black_box("enlarge"))));
    c.bench_function("remove_init_chars enlarge 2", |b| b.iter(|| remove_init_chars(black_box("enlarge"), black_box(2))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
