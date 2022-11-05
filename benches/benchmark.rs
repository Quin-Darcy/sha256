use shasher::utils::msg::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn get_binary_bench(c: &mut Criterion) {
    let input: &str = "/home/arbegla/Projects/Rust/libraries/shasher/src/lib.rs";
    c.bench_function(
        "get_binary", 
        |b| b.iter(|| get_binary(input))
    );
}

fn get_padding_bench(c: &mut Criterion) {
    let input: u32 = 417;
    c.bench_function(
        "get_padding",
        |b| b.iter(|| get_padding(input))
    );
}

fn get_parsed_message_bench(c: &mut Criterion) {
    let path: &str = "/home/arbegla/Projects/Rust/libraries/shasher/src/lib.rs";
    let input: Vec<u32> = match get_binary(path) {
        Ok(b) => b,
        Err(_e) => panic!("Error. Could not get bytes from {}", path),
    };
    c.bench_function(
        "get_parsed_message",
        |b| b.iter(|| get_parsed_message(&input))
    );
}

criterion_group!(
    benches, 
    get_binary_bench,
    get_padding_bench,
    get_parsed_message_bench
);
criterion_main!(benches);
