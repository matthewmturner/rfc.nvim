use criterion::{criterion_group, criterion_main, Criterion};

fn build_term_frequencies(contents: String) {
    let mut index = rfsee_tf_idf::TfIdf::default();
    let rfc = rfsee_tf_idf::RfcEntry {
        number: 1,
        url: "https://rfsee.com".to_string(),
        title: "RFC 8124".to_string(),
        content: Some(contents),
    };
    index.add_rfc_entry(rfc);
}

fn criterion_benchmark(c: &mut Criterion) {
    let contents = std::fs::read_to_string("../../data/rfc_8124.txt").unwrap();
    let mut group = c.benchmark_group("index");
    group.sample_size(10);
    group.bench_function("build_index", |b| {
        b.iter(|| build_term_frequencies(contents.clone()))
    });
    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
