use std::thread;

pub fn start_move_basic() {
    // 自动 copy
    let x = 100;

    let handle1 = thread::spawn(move || {
        println!("Hello from a thread with move, x={}!", x);
    });

    handle1.join().unwrap();

    let handle2 = thread::spawn(move || {
        println!("Hello from a thread with move again, x={}!", x);
    });

    handle2.join().unwrap();

    let handle3 = thread::spawn(|| {
        println!("Hello from a thread without move");
    });

    handle3.join().unwrap();
}

#[allow(dead_code)]
fn start_move_arr() {
    let x = vec![1, 2, 3];
    let handle1 = thread::spawn(move || {
        println!("Hello from a thread with move, x={:?}!", x);
    });

    handle1.join().unwrap();

    // let handle2 = thread::spawn(move || {
    //     println!("Hello from a thread with move again, x={:?}!", x);
    // });

    // handle2.join().unwrap();

    let handle3 = thread::spawn(|| {
        println!("Hello from a thread without move");
    });

    handle3.join().unwrap();
}
