use criterion::{black_box, criterion_group, criterion_main, Criterion};
use emdb_lib::words::beginnings::{init_en_in_to_n, init_un_to_u, em_um_to_m};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("init_en_in_to_n enlarge", |b| b.iter(|| init_en_in_to_n(black_box("enlarge"))));
    c.bench_function("init_un_to_u enlarge", |b| b.iter(|| init_un_to_u(black_box("enlarge"))));
    c.bench_function("init_im_to_i imagery", |b| b.iter(|| init_un_to_u(black_box("imagery"))));
    c.bench_function("em_um_to_m umpire", |b| b.iter(|| em_um_to_m(black_box("umpire"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
