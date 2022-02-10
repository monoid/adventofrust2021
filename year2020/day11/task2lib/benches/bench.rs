use std::io;

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = io::BufReader::new(std::fs::File::open("../input").expect("Input file not found"));
    let map = lib::read_map(input);
    c.bench_function("task2 on input", |b| b.iter(|| task2lib::solve(&map)));
    c.bench_function("task2 init", |b| b.iter(|| task2lib::FlatMap::from_2d(&map)));
    c.bench_function("task2 vis_map", |b| b.iter(|| task2lib::produce_visibility_map(&task2lib::FlatMap::from_2d(&map))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
