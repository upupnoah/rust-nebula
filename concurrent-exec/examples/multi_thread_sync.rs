use std::thread;
use std::time::Duration;

fn main() {
    println!("多线程同步：开始处理任务...");

    let handle1 = thread::spawn(|| {
        println!("线程 A：开始长耗时操作...");
        thread::sleep(Duration::from_secs(2));
        println!("线程 A：耗时操作完成！");
    });

    let handle2 = thread::spawn(|| {
        println!("线程 B：开始长耗时操作...");
        thread::sleep(Duration::from_secs(4));
        println!("线程 B：耗时操作完成！");
    });

    // 在主线程阻塞等待子线程执行完
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("所有子线程任务完成（多线程同步）！");
}
