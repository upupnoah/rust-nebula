use tokio::time::{interval, sleep, Duration, Instant};

#[tokio::main] // 默认使用多线程调度器
async fn main() {
    println!("[Main] 启动多线程异步运行时，开始处理任务...");

    // 任务1：模拟耗时 2s 的工作
    let handle1 = tokio::spawn(async {
        println!("[Task1] 开始长耗时操作 (2s sleep)...");
        sleep(Duration::from_secs(2)).await;
        println!("[Task1] 2s 耗时操作完成！");
    });

    // 任务2：模拟耗时 4s 的工作
    let handle2 = tokio::spawn(async {
        println!("[Task2] 开始长耗时操作 (4s sleep)...");
        let start = Instant::now();
        sleep(Duration::from_secs(4)).await;
        println!("[Task2] 4s 耗时操作完成，实际用时: {:.2?}", start.elapsed());
    });

    // 任务3：间歇打印日志，持续 5 次，每次间隔 1s
    let handle3 = tokio::spawn(async {
        println!("[Logger] 开始定时打印日志...");
        let mut ticker = interval(Duration::from_secs(1));
        for i in 1..=5 {
            ticker.tick().await; // 等待 1 秒
            println!("[Logger] 第 {i} 次打印日志 (1s 间隔)");
        }
        println!("[Logger] 定时打印日志结束");
    });

    // 任务4：短暂任务，每次 sleep 500ms，重复 3 次
    let handle4 = tokio::spawn(async {
        println!("[Task4] 多次短耗时操作 (3次, 每次500ms) 开始...");
        for i in 1..=3 {
            sleep(Duration::from_millis(500)).await;
            println!("[Task4] 第 {i} 次短暂操作完成 (500ms)");
        }
        println!("[Task4] 短耗时操作全部完成");
    });

    // 并发等待所有异步任务结束
    let _ = tokio::join!(handle1, handle2, handle3, handle4);

    println!("[Main] 所有异步任务完成（多线程异步）！");
}
