mod radix_sort;

use std::env;
use rand::Rng;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <number of elements> <max value>", args[0]);
        return;
    }
    let n: usize = args[1].parse().expect("Number of elements should be a positive integer");
    let max: u32 = args[2].parse().expect("Max value should be a positive integer");
    let mut random_numbers = generate_random_numbers(n, max);
    let start = Instant::now();
    radix_sort::radix_sort(&mut random_numbers, max);
    let end = start.elapsed();
    println!("{} us", end.as_micros());
}

fn generate_random_numbers(size: usize, max: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut numbers = Vec::new();
    numbers.reserve(size);
    for _ in 0..size {
        numbers.push(rng.gen_range(0..max));
    }
    numbers
}