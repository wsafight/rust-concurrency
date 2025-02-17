use std::thread;

// 如果你只是想让渡出时间片，你不用设置 sleep 0，
// 而是调用 yield_now 函数
pub fn start_thread_with_yield_now() {
    let handle1 = thread::spawn(|| {
        thread::yield_now();
        println!("yield 01")
    });

    let handle2 = thread::spawn(|| {
        thread::yield_now();
        println!("yield 02")
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
