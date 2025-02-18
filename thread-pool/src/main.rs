use std::time::Instant;

mod barrier;
mod create;
mod fast_thread_pool;
mod fib;
mod rusty_pool;
mod scoped_threadpool;
mod thread_pool;

fn main() {
    create::create();
    let start = Instant::now();
    let n1 = fib::fib(10);
    let duration = start.elapsed();
    println!("fib took {} milliseconds", duration.as_millis());
    println!("{}", n1);
    let n2 = fib::fib_rayon(10);
    println!("{}", n2);

    thread_pool::use_thread_pool();
    barrier::barrier_demo();
    rusty_pool::rusty_pool_example();
    rusty_pool::rusty_pool_example3();
    let _ = fast_thread_pool::use_fast_threadpool();
    scoped_threadpool::use_scoped_threadpool();
}
