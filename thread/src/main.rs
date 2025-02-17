mod current_thread;
mod parallelism_count;
mod park;
mod scoped;
mod spawn_uncheck;
mod start_thread;
mod thread_build;
mod yield_now;
mod thread_local;

fn main() {
    start_thread::start_thread();
    thread_build::start_one_thread_by_builder();
    spawn_uncheck::thread_spawn_uncheck();
    current_thread::current_thread();
    park::park_unpark();
    parallelism_count::get_parallelism_count();
    yield_now::start_thread_with_yield_now();
    scoped::start_scoped_threads();
}
