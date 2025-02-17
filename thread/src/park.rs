use std::{thread, time::Duration};

pub fn park_unpark() {
    let parked_thread = thread::spawn(|| {
        println!("Parking thread");
        thread::park();
        println!("Thread unparked");
    });

    let unpark_thread = thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        // thread.upark 方法以原子方式使令牌可用
        parked_thread.thread().unpark();
        parked_thread.join().unwrap();
    });

    unpark_thread.join().unwrap();

    // 如果先调用 unpark , 接下来的那个 park 会立即返回
    // 注： 一个线程只有一个令牌，这个令牌或者存在或者只有一个
}
