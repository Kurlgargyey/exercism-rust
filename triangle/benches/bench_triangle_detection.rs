use criterion::Criterion;
use criterion::{ criterion_group, criterion_main };

use triangle::triangle_tests::*;

fn bench_triangle_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("Different ways to test sides for triangle validity");
    group.bench_function("trigonometry", |b| b.iter(|| triangle_test(&[20, 30, 40])));
    group.bench_function("summing", |b| b.iter(|| triangle_test_alternative(&[20, 30, 40])));
    group.finish();
}

criterion_group!(benches, bench_triangle_detection);
criterion_main!(benches);
