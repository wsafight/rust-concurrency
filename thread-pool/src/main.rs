use std::time::Instant;

mod create;
mod fib;

fn main() {
    create::create();
    let start = Instant::now();
    let n1 = fib::fib(40);
    let duration = start.elapsed();
    println!("fib took {} milliseconds", duration.as_millis());
    println!("{}", n1);
    let n2 = fib::fib_rayon(40);
    println!("{}", n2);
}
