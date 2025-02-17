use std::thread::Builder;

pub fn thread_spawn_uncheck() {
    let builder = Builder::new();

    let x = 1;
    let thread_x = &x;

    let handler = unsafe {
        // 更宽松的声明周期的绑定
        builder
            .spawn_unchecked(move || println!("x = {}", *thread_x))
            .unwrap()
    };

    handler.join().unwrap()
}
