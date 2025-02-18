use scoped_threadpool::Pool;

pub fn use_scoped_threadpool() {
    let mut pool = Pool::new(4);

    let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7];

    pool.scoped(|s| {
        for e in &mut vec {
            s.execute(move || {
                *e += 1;
            });
        }
    });

    println!("{:?}", vec);
}
