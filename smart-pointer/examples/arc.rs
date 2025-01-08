// Arc使用场景：
// 1. 在多线程环境下共享只读数据
// 2. 需要线程安全的引用计数
// 3. 当多个线程需要访问相同的数据，但不需要修改它时
//
// 实际应用场景：
// 1. 数据库连接池：多个工作线程共享同一个连接配置
//    例如：let pool = Arc::new(DatabasePool::new(config));
// 2. 应用配置：在多线程服务器中共享配置信息
//    例如：let settings = Arc::new(ServerConfig::load());
// 3. 共享缓存：多个处理线程访问同一个只读缓存
//    例如：let cache = Arc::new(LruCache::new());
use std::sync::Arc;
use std::thread;

struct Config {
    setting: i32,
}

fn main() {
    let config = Arc::new(Config { setting: 42 });

    let mut handles = vec![];
    for i in 0..5 {
        let config_clone = Arc::clone(&config);
        let handle = thread::spawn(move || {
            println!("Thread {}", i);
            println!("Config setting: {}", config_clone.setting);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
