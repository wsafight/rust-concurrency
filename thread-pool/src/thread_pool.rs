use std::sync::mpsc::channel;

use threadpool::ThreadPool;

pub fn use_thread_pool() {
    // 创建一个包含 4 个线程的线程池
    let pool = ThreadPool::new(4);

    let (sender, receiver) = channel();

    for i in 0..8 {
        let sender = sender.clone();
        pool.execute(move || {
            let result = i * 2;
            sender.send(result).expect("发送失败");
        });
    }

    for _ in 0..8 {
        let result = receiver.recv().expect("接收失败");
        println!("任务结果： {}", result);
    }
}
