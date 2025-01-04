use criterion::{criterion_group, criterion_main, Criterion};

fn build_index() {
    let mut index = rfsee_tf_idf::TfIdf::default();
    index.par_load_rfcs().unwrap();
    index.finish();
    let path = std::path::PathBuf::from("/tmp/bench_index.json");
    index.save(&path)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("index");
    group.sample_size(10);
    group.bench_function("build_index", |b| b.iter(build_index));
    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
