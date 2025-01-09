use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("开始处理同步任务...");

    let result1 = long_running_task(1);
    println!("任务 {} 完成，结果是 {}", 1, result1);

    let result2 = long_running_task(2);
    println!("任务 {} 完成，结果是 {}", 2, result2);

    let duration = start.elapsed();
    println!("所有同步任务完成，耗时：{:?}", duration);
}

fn long_running_task(id: usize) -> i32 {
    println!("任务 {} 开始", id);
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("任务 {} 结束", id);
    id as i32 * 10
}
