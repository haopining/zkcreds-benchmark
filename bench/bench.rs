extern crate criterion;
extern crate zkcreds_benchmark;
mod util;


use criterion::{criterion_group, criterion_main};
use zkcreds_benchmark::simple_expiry::bench_expiry;
use util::new_size_file as setup; // Gotta set up logging proof sizes to CSV

criterion_group!(benches, bench_expiry);
criterion_main!(setup, benches);