use criterion::{black_box, criterion_group, criterion_main, Criterion};
use emdb_lib::{
    NumeralTransform, PhonemeClusterTransform, PrefixTransform, Speedwriter, SuffixTransform,
    TenseOmissionTransform, Transform, WordBuildingTransform, WordSignTransform,
};

pub fn benchmark_speedwriter(c: &mut Criterion) {
    let speedwriter = Speedwriter::new();
    let sample_text = "This is a sample text with multiple words to test the speedwriting transformation system. \
                      It contains words with common prefixes like unmodified, enlarge, information, and \
                      suffixes like quickly, running, happily, and literary.";

    c.bench_function("speedwriter_full_text", |b| {
        b.iter(|| speedwriter.to_speedwriting(black_box(sample_text)))
    });
}

pub fn benchmark_transforms(c: &mut Criterion) {
    let prefix_transform = PrefixTransform::new();
    let suffix_transform = SuffixTransform::new();
    let phoneme_transform = PhonemeClusterTransform::new();
    let word_building_transform = WordBuildingTransform::new();
    let word_sign_transform = WordSignTransform::new();
    let numeral_transform = NumeralTransform::new();
    let tense_transform = TenseOmissionTransform::new();

    let mut group = c.benchmark_group("transforms");

    // Benchmark prefix transformations
    group.bench_function("prefix_enlarge", |b| {
        b.iter(|| prefix_transform.apply(black_box("enlarge")))
    });

    group.bench_function("prefix_information", |b| {
        b.iter(|| prefix_transform.apply(black_box("information")))
    });

    group.bench_function("prefix_unmodified", |b| {
        b.iter(|| prefix_transform.apply(black_box("unmodified")))
    });

    // Benchmark suffix transformations
    group.bench_function("suffix_quickly", |b| {
        b.iter(|| suffix_transform.apply(black_box("quickly")))
    });

    group.bench_function("suffix_running", |b| {
        b.iter(|| suffix_transform.apply(black_box("running")))
    });

    group.bench_function("suffix_literary", |b| {
        b.iter(|| suffix_transform.apply(black_box("literary")))
    });

    // Benchmark phoneme transformations
    group.bench_function("phoneme_transfer", |b| {
        b.iter(|| phoneme_transform.apply(black_box("transfer")))
    });

    group.bench_function("phoneme_strangle", |b| {
        b.iter(|| phoneme_transform.apply(black_box("strangle")))
    });

    // Benchmark word building transformations
    group.bench_function("word_building_together", |b| {
        b.iter(|| word_building_transform.apply(black_box("together")))
    });

    group.bench_function("word_sign_therefore", |b| {
        b.iter(|| word_sign_transform.apply(black_box("therefore")))
    });

    group.bench_function("numeral_three", |b| {
        b.iter(|| numeral_transform.apply(black_box("three")))
    });

    group.bench_function("tense_worked", |b| {
        b.iter(|| tense_transform.apply(black_box("worked")))
    });

    group.finish();
}

pub fn compare_legacy_vs_optimized(c: &mut Criterion) {
    let mut group = c.benchmark_group("legacy_vs_optimized");

    // Test prefix transformations
    group.bench_function("legacy_init_en_in_to_n", |b| {
        b.iter(|| emdb_lib::words::beginnings::init_en_in_to_n(black_box("enlarge")))
    });

    group.bench_function("optimized_prefix_transform", |b| {
        let transform = PrefixTransform::new();
        b.iter(|| transform.apply(black_box("enlarge")))
    });

    // Test suffix transformations
    group.bench_function("legacy_final_ly_ily_to_l", |b| {
        b.iter(|| emdb_lib::words::endings_freq::final_ly_ily_to_l(black_box("quickly")))
    });

    group.bench_function("optimized_suffix_transform", |b| {
        let transform = SuffixTransform::new();
        b.iter(|| transform.apply(black_box("quickly")))
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_speedwriter,
    benchmark_transforms,
    compare_legacy_vs_optimized
);
criterion_main!(benches);
