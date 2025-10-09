//! Performance benchmarks for py-import-helper
//!
//! This module contains benchmarks to measure the performance of import
//! categorization and formatting operations using Criterion.

use criterion::{criterion_group, criterion_main, Criterion};
use py_import_helper::ImportHelper;
use std::hint::black_box;

fn benchmark_import_categorization(c: &mut Criterion) {
    c.bench_function("categorize 100 imports", |b| {
        b.iter(|| {
            let mut helper = ImportHelper::new();
            for i in 0..100 {
                helper.add_import_string(&format!("from typing import Type{}", black_box(i)));
                helper.add_import_string(&format!("import module{}", black_box(i)));
                helper
                    .add_type_checking_import(&format!("from package{} import Item", black_box(i)));
            }
            helper.get_all_categorized()
        })
    });
}

fn benchmark_formatting(c: &mut Criterion) {
    let mut helper = ImportHelper::new();

    // Pre-populate with many imports
    for i in 0..100 {
        helper.add_import_string(&format!("from typing import Type{}", i));
        helper.add_import_string(&format!("import module{}", i));
    }

    c.bench_function("format 200 imports", |b| {
        b.iter(|| black_box(helper.get_formatted()))
    });
}

criterion_group!(
    benches,
    benchmark_import_categorization,
    benchmark_formatting
);
criterion_main!(benches);
