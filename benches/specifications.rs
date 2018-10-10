#[macro_use]
extern crate criterion;

use criterion::Criterion;
use dice_spec_perf::*;

static EXPRESSIONS: &[&str] = &["2d6", "17", "1d2d3", "hello"];

fn benchmarks(c: &mut Criterion) {
    c.bench_function("Split", |b| {
        b.iter(|| {
            for expression in EXPRESSIONS {
                criterion::black_box(expression.parse::<SplitSpecification>());
            }
        })
    });

    c.bench_function("Regex", |b| {
        b.iter(|| {
            for expression in EXPRESSIONS {
                criterion::black_box(expression.parse::<RegexSpecification>());
            }
        })
    });

    c.bench_function("Pest", |b| {
        b.iter(|| {
            for expression in EXPRESSIONS {
                criterion::black_box(expression.parse::<PestSpecification>());
            }
        })
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
