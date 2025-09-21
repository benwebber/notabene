use criterion::{Criterion, criterion_group, criterion_main};
use notabene::{Linter, parse};

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("Keep a Changelog 1.1.1", |b| {
        b.iter(|| Linter::default().lint(&parse(include_str!("data/keep-a-changelog-1.1.1.md"))))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
