use std::thread;

fn start_one_thread() {
    // 在当前线程中启动一个新的线程
    let handle = thread::spawn(|| {
        println!("Hello from a thread");
        "done"
    });

    // 等待线程完成
    // handle.join().unwrap()
    // match handle.join() {
    //     Ok(v) => println!("thread result: {}", v),
    //     Err(e) => println!("error {:?}", e)
    // }
    if let Ok(v) = handle.join() {
        println!("thread done: {}", v);
    }
}

fn start_two_thread() {
    let handle1 = thread::spawn(|| {
        println!("Hello from a thread1");
    });
    let handle2 = thread::spawn(|| {
        println!("Hello from a thread2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn start_n_thread() {
    const N: u32 = 10;
    let handles = (0..N).map(|i| {
        thread::spawn(move || {
            println!("Hello from a thread{}", i);
        })
    });

    for handle in handles {
        handle.join().unwrap()
    }
}

pub fn start_thread() {
    start_one_thread();
    start_two_thread();
    start_n_thread();
}
