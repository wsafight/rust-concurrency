// mod affinity;
mod current_thread;
mod parallelism_count;
mod park;
mod scoped;
mod spawn_uncheck;
mod start_thread;
mod thread_build;
mod thread_control;
mod thread_local;
mod thread_move;
mod thread_priority;
mod yield_now;
mod panic;

fn main() {
    start_thread::start_thread();
    thread_build::start_one_thread_by_builder();
    spawn_uncheck::thread_spawn_uncheck();
    current_thread::current_thread();
    park::park_unpark();
    parallelism_count::get_parallelism_count();
    yield_now::start_thread_with_yield_now();
    scoped::start_scoped_threads();
    thread_local::start_threads_with_threadlocal();
    thread_move::start_move_basic();
    thread_control::thread_control();
    thread_priority::start_thread_with_priority();
    // 业务绑核，macos 不支持
    // affinity::use_affinity();
    panic::panic_example();
    panic::panic_caught_example();
}
