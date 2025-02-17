use std::{cell::RefCell, thread};

pub fn start_threads_with_threadlocal() {
    thread_local!(static COUNTER: RefCell<u32> = RefCell::new(1));

    COUNTER.with(|c| {
        *c.borrow_mut() = 2;
    });

    let handle1 = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() = 3;
        });

        COUNTER.with(|c| {
            println!("Hello from a thread7, c={}!", *c.borrow());
        });
    });

    let handle2 = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() = 4;
        });

        COUNTER.with(|c| {
            println!("Hello from a thread, c = {}", *c.borrow());
        })
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    COUNTER.with(|c| {
        println!("Hello from main, c= {}", *c.borrow());
    })
}
