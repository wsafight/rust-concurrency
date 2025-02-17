// 绑核是在极端情况提升性能的有效手段之一
// 让这些核专门提供给我们的业务服务，既提供了 CPU 资源隔离，还提升了性能

// #[cfg(not(target_os = "macos"))]
// pub fn use_affinity() {
//     let cores: Vec<usize> = (0..affinity::get_core_num()).step_by(2).collect();

//     println!("Binding thread to cores : {:?}", &cores);

//     affinity::set_thread_affinity(&cores).unwrap();
//     println!(
//         "Current thread affinity : {:?}",
//         affinity::get_thread_affinity().unwrap()
//     );
// }
