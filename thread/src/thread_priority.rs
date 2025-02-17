// use std::convert::TryInto;
use std::thread;
use thread_priority::*;

pub fn start_thread_with_priority() {
    let handle1 = thread::spawn(|| {
        assert!(set_current_thread_priority(ThreadPriority::Min).is_ok());
        println!("666");
    });

    let handle2 = thread::spawn(|| {
        assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
        println!("777");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // assert!(
    //     set_current_thread_priority(ThreadPriority::Crossplatform(0.try_into().unwrap())).is_ok()
    // );

    let thread = ThreadBuilder::default()
        .name("MyThread")
        .priority(ThreadPriority::Min)
        .spawn(|res| {
            println!("Set priority result: {:?}", res);
            assert!(res.is_ok());
        })
        .unwrap();

    let thread2 = ThreadBuilder::default()
        .name("MyThread2")
        .priority(ThreadPriority::Max)
        .spawn_careless(|| {
            println!("We don't care about the priority result.");
        })
        .unwrap();

    thread.join().unwrap();
    thread2.join().unwrap();
}
