use shasher::utils::msg::{get_binary, get_parsed_message};
use shasher::utils::compute::converter::num_to_bin;
use criterion::{criterion_group, criterion_main, Criterion};



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

fn num_to_bin_bench(c: &mut Criterion) {
    let num: u32 = 0;
    let num_of_bits: u32 = 8;
    c.bench_function(
        "num_to_bin",
        |b| b.iter(|| num_to_bin(num, num_of_bits))
    );
}

criterion_group!(
    benches, 
    //get_parsed_message_bench
    num_to_bin_bench
);
criterion_main!(benches);
