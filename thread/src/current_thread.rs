use std::thread::{self, Builder};

pub fn current_thread() {
    let current_thread = thread::current();
    println!(
        "current thread: {:?},{:?}",
        current_thread.id(),
        current_thread.name()
    );

    let builder = Builder::new().name("foo".into()).stack_size(32 * 1024);

    let handler = builder
        .spawn(|| {
            let current_thread = thread::current();
            println!(
                "current thread: {:?},{:?}",
                current_thread.id(),
                current_thread.name()
            );
        })
        .unwrap();

    handler.join().unwrap();
}
