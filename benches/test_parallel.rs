use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use ::radix_sort::radix_sort;
use ::radix_sort::radix_sort_parallel;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

const SIZE: usize = 10_000_000;
const MAX: u32 = 1021;

fn generate_random_numbers(size: usize, max: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..max)).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("seq", |b|
        b.iter(|| {
            let mut random_numbers = generate_random_numbers(SIZE, MAX);
            radix_sort::radix_sort(&mut random_numbers, MAX);
        }));

    c.bench_function("par", |b|
        b.iter(|| {
            let mut random_numbers = generate_random_numbers(SIZE, MAX);
            radix_sort_parallel::radix_sort(&mut random_numbers, MAX);
        }));
}

criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);