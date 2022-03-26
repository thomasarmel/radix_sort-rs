#![feature(test)]

pub mod radix_sort;
pub mod radix_sort_parallel;

use rand::Rng;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <number of elements> <max value>", args[0]);
        return;
    }
    let n: usize = args[1]
        .parse()
        .expect("Number of elements should be a positive integer");
    let max: u32 = args[2]
        .parse()
        .expect("Max value should be a positive integer");
    let mut random_numbers = generate_random_numbers(n, max);
    let start = Instant::now();
    radix_sort::radix_sort(&mut random_numbers, max);
    let end = start.elapsed();
    println!("{} us", end.as_micros());

    let mut random_numbers = generate_random_numbers(n, max);
    let start = Instant::now();
    radix_sort_parallel::radix_sort(&mut random_numbers, max);
    let end = start.elapsed();
    println!("{} us", end.as_micros());
}

fn generate_random_numbers(size: usize, max: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..max)).collect()
}

#[cfg(test)]
mod tests
{
    extern crate test;
    use test::Bencher;

    use super::*;

    const SIZE: usize = 10_000;
    const MAX: u32 = 1021;
/*
    #[bench]
    fn test_radix_sort(b: &mut Bencher) {
        b.iter(|| {
            let mut random_numbers = generate_random_numbers(SIZE, MAX);
            radix_sort::radix_sort(&mut random_numbers, MAX);
        });
    }

    #[bench]
    fn test_radix_sort_parallel(b: &mut Bencher) {
        b.iter(|| {
            let mut random_numbers = generate_random_numbers(SIZE, MAX);
            radix_sort_parallel::radix_sort(&mut random_numbers, MAX);
        });
    }
*/
}