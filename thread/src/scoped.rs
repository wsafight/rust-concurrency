use std::thread;

pub fn start_scoped_threads() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    // 所以不需要考虑并发
    thread::scope(|s| {
        s.spawn(|| {
            println!("hello scoped1");
            // a 是只读
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello scoped2");
            // x 是只写
            x = a[0] + 3
        });
    });

    println!("hello from the main thread");

    // a 是只写
    a.push(4);
    // x 是只读
    assert_eq!(x - 1, a.len());
}
