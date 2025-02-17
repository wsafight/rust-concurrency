use std::{panic, thread, time::Duration};

pub fn panic_example() {
    println!("Hello,world");

    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        panic!("boom");
    });

    match handle.join() {
        Ok(v) => {
            println!("All is well! {:?}", v)
        },
        Err(e) => {
            println!("Got an error! {:?}", e)
        }
    }

    println!("Exiting main")
}


pub fn panic_caught_example() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        let result = panic::catch_unwind(|| {
            panic!("boom");
        });
        println!("panic caught, result = {}", result.is_err());
    });

    match handle.join() {
        Ok(v) => {
            println!("All is well! {:?}", v)
        },
        Err(e) => {
            println!("Got an error! {:?}", e)
        }
    }

    println!("Exiting main")
}