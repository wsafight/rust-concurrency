use std::{
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use rusty_pool::ThreadPool;

pub fn rusty_pool_example() {
    let pool = ThreadPool::default();

    for _ in 1..10 {
        pool.execute(|| {
            println!("Hello from a rusty_pool");
        });
    }

    pool.join();

    let handle = pool.evaluate(|| {
        thread::sleep(Duration::from_secs(5));
        4
    });
    let result = handle.await_complete();
    println!("rusty evaluate result is {}", result);
}

// pub fn rusty_pool_example2 () {
//     let pool = ThreadPool::default();

//     let handle = pool.complete(async {
//         let a = some_async_f(4, 6).await;
//     });

// }

pub fn rusty_pool_example3() {
    let pool = ThreadPool::default();

    for _ in 0..10 {
        pool.execute(|| thread::sleep(Duration::from_secs(2)));
    }

    pool.join_timeout(Duration::from_secs(1));

    let count = Arc::new(AtomicI32::new(0));

    for _ in 0..15 {
        let count_temp = count.clone();
        pool.execute(move || {
            thread::sleep(Duration::from_secs(1));
            count_temp.fetch_add(1, Ordering::SeqCst);
        });
    }

    pool.shutdown_join();
    println!("current count is {}", count.load(Ordering::SeqCst))
}
