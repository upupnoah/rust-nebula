// use std::time::Instant;
// use tokio::time::{sleep, Duration};

// #[tokio::main(flavor = "current_thread")]
// async fn main() {
//     let start = Instant::now();
//     println!("开始处理异步任务...");

//     // 同时启动两个异步任务
//     let task1 = long_running_task(1);
//     let task2 = long_running_task(2);

//     // 并发等待两个任务完成
//     let (result1, result2) = tokio::join!(task1, task2);

//     println!("任务 {} 完成，结果是 {}", 1, result1);
//     println!("任务 {} 完成，结果是 {}", 2, result2);

//     let duration = start.elapsed();
//     println!("所有异步任务完成，耗时：{:?}", duration);
// }

// async fn long_running_task(id: usize) -> i32 {
//     println!("任务 {} 开始", id);
//     sleep(Duration::from_secs(2)).await;
//     println!("任务 {} 结束", id);
//     id as i32 * 10
// }

use std::time::Instant;
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    println!("开始处理异步任务...");

    // 同时启动两个异步任务
    let handle1 = tokio::spawn(long_running_task(1));

    let handle2 = tokio::spawn(long_running_task(2));

    let temp_val = 3_i32 * 10;
    let handle3 = tokio::spawn(async move {
        println!("任务 {} 开始", 3);
        sleep(Duration::from_secs(2)).await;
        println!("任务 {} 结束", 3);
        temp_val
    });

    // 等待任务完成
    let (Ok(result1_val), Ok(result2_val), Ok(result3_val)) =
        tokio::join!(handle1, handle2, handle3)
    else {
        panic!("某个任务出错了");
    };

    println!("任务 {} 完成，结果是 {}", 1, result1_val);
    println!("任务 {} 完成，结果是 {}", 2, result2_val);
    println!("任务 {} 完成，结果是 {}", 3, result3_val);

    let duration = start.elapsed();
    println!("所有异步任务完成，耗时：{:?}", duration);
}

async fn long_running_task(id: usize) -> i32 {
    println!("任务 {} 开始", id);
    sleep(Duration::from_secs(2)).await;
    println!("任务 {} 结束", id);
    id as i32 * 10
}
