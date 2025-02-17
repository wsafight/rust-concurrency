use std::thread::Builder;

pub fn start_one_thread_by_builder() {
    let builder = Builder::new()
        // 线程名称
        .name("foo".into())
        // 栈大小
        .stack_size(32 * 1024);
    let handler = builder
        .spawn(|| {
            println!("Hello form a thread by builder");
        })
        .unwrap();

    handler.join().unwrap();
}
