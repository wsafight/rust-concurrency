use rayon::ThreadPoolBuilder;

pub fn create() {
    ThreadPoolBuilder::new()
        .thread_name(|i| format!("worker-{}", i))
        // 指定线程池的线程数，默认 cpu 内核数
        .num_threads(8)
        .build()
        .unwrap();
    println!("创建了线程池")

    // 初始化全局的线程池,只会初始化一次
    // ThreadPoolBuilder::new()
    //     .num_threads(22)
    //     .build_global()
    //     .unwrap();
}
