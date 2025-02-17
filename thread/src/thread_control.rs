use std::{thread, time::Duration};
use thread_control::*;

pub fn thread_control() {
    let (flag, control) = make_pair();

    let handle = thread::spawn(move || {
        while flag.alive() {
            thread::sleep(Duration::from_millis(10));
            println!("I'm alive!")
        }
    });

    thread::sleep(Duration::from_millis(100));

    assert_eq!(control.is_done(), false);

    control.stop();
    handle.join().unwrap();
    assert_eq!(control.is_interrupted(), false);

    assert_eq!(control.is_done(), true)
}
