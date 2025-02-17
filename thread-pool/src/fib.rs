use std::time::Instant;

use rayon::{join, ThreadPoolBuilder};

pub fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }

    return fib(n - 1) + fib(n - 2);
}

fn fibRayon(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    let (a, b) = join(|| fibRayon(n - 1), || fibRayon(n - 2));
    return a + b;
}

pub fn fib_rayon(n: usize) -> usize {
    let pool = ThreadPoolBuilder::new()
    .num_threads(12)
    .build()
    .unwrap();

    let start = Instant::now();
    let result = pool.install(|| fibRayon(n));
    let duration = start.elapsed();
    println!("fib_rayon took {} milliseconds", duration.as_millis());
    return result;
}
