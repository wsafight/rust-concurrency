// 获取并发的能力值

use std::thread;

pub fn get_parallelism_count() {
    let count = thread::available_parallelism().unwrap().get();
    println!("available_parallelism count: {}", count)
}
