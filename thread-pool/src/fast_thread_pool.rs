// 保证你在执行你的任务之前不会支付线程生成的成本。
// 新线程仅在工作线程的‘‘闲置时间’’（例如，在返回作业结果后）期间生成

use fast_threadpool::{ThreadPool, ThreadPoolConfig, ThreadPoolDisconnected};

pub fn use_fast_threadpool() -> Result<(), ThreadPoolDisconnected> {
    let pool = ThreadPool::start(ThreadPoolConfig::default(), ()).into_sync_handler();

    println!("{}", pool.execute(|_| { 2 + 2 })?);
    Ok(())
}
