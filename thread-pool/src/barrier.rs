use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Barrier,
};

use threadpool::ThreadPool;
// 任务数一定不能大于 worker 的数量，否则会导致死锁

pub fn barrier_demo() {
    let n_worker = 42;
    let n_jobs = 23;

    let pool = ThreadPool::new(n_worker);
    let an_atomic = Arc::new(AtomicUsize::new(0));

    // 创建一个 barrier，等待所有任务完成
    let barrier = Arc::new(Barrier::new(n_jobs + 1));

    for _ in 0..n_jobs {
        let barrier = barrier.clone();
        let an_atomic = an_atomic.clone();

        pool.execute(move || {
            // 执行一个很重的任务
            an_atomic.fetch_add(1, Ordering::Relaxed);

            // 等待其他线程完成
            barrier.wait();
        });
    }

    barrier.wait();

    println!("barrier demo {}", an_atomic.load(Ordering::SeqCst));
    assert_eq!(an_atomic.load(Ordering::SeqCst), 23);
}
