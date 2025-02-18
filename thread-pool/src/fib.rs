use std::time::Instant;

use rayon::{join, ThreadPoolBuilder};

pub fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

fn i_fib_rayon(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    let (a, b) = join(|| i_fib_rayon(n - 1), || i_fib_rayon(n - 2));
    a + b
}

pub fn fib_rayon(n: usize) -> usize {
    let pool = ThreadPoolBuilder::new().num_threads(12).build().unwrap();

    let start = Instant::now();
    let result = pool.install(|| i_fib_rayon(n));
    let duration = start.elapsed();
    println!("fib_rayon took {} milliseconds", duration.as_millis());
    result
}
