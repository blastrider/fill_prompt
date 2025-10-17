#![cfg(feature = "bench")]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

fn bench_fill(c: &mut Criterion) {
    let tpl = "Line {{a}} middle {{b}} end {{c}} ".repeat(200);
    let mut vars = HashMap::new();
    vars.insert("a".to_string(), "AAA".to_string());
    vars.insert("b".to_string(), "BBB".to_string());
    vars.insert("c".to_string(), "CCC".to_string());

    c.bench_function("fill_template large", |b| {
        b.iter(|| {
            let _ = fill_prompt::fill_template(black_box(&tpl), vars.clone()).unwrap();
        })
    });
}

criterion_group!(benches, bench_fill);
criterion_main!(benches);
