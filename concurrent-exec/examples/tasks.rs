use rand::Rng;
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("开始并发执行多个任务...");

    // 创建一个 broadcast channel 用于发送取消信号
    let (tx, _) = broadcast::channel(3); // 3 是接收者数量

    // 启动多个相同的任务
    let handle1 = tokio::spawn(process_task(1, tx.subscribe()));
    let handle2 = tokio::spawn(process_task(2, tx.subscribe()));
    let handle3 = tokio::spawn(process_task(3, tx.subscribe()));

    // 等待任意一个任务完成
    tokio::select! {
        Ok(result) = handle1 => {
            println!("任务1 首先完成，结果: {}", result);
            // 发送取消信号给其他任务
            let _ = tx.send(());
        }
        Ok(result) = handle2 => {
            println!("任务2 首先完成，结果: {}", result);
            let _ = tx.send(());
        }
        Ok(result) = handle3 => {
            println!("任务3 首先完成，结果: {}", result);
            let _ = tx.send(());
        }
    }

    println!("所有任务已结束！");
}

async fn process_task(id: u32, mut rx: broadcast::Receiver<()>) -> i32 {
    println!("任务 {} 开始执行", id);

    loop {
        tokio::select! {
            // 检查是否收到取消信号
            Ok(_) = rx.recv() => {
                println!("任务 {} 收到取消信号，正在退出", id);
                return -1;
            }
            // 模拟一些处理过程
            _ = sleep(Duration::from_secs(1)) => {
                let result = rand::thread_rng().gen_range(1..100);
                if result > 90 {
                    println!("任务 {} 找到结果: {}", id, result);
                    return result;
                }
                println!("任务 {} 继续寻找...", id);
            }
        }
    }
}
