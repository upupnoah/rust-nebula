// RefCell使用场景：
// 1. 当需要在不可变引用中修改数据时使用
// 2. 用于实现内部可变性模式
// 3. 适用于单线程环境下需要运行时借用检查的场景
//
// 实际应用场景：
// 1. Mock对象在测试中：允许修改mock对象的内部状态
//    例如：mock_object.expect_call().times(3)
// 2. 观察者模式：在回调函数中修改状态
//    例如：widget.on_click(|| self.update_state())
// 3. 缓存实现：需要在查询时更新缓存状态
//    例如：cache.get_or_insert(compute_value())
use std::cell::RefCell;

struct Counter {
    count: RefCell<i32>,
}

impl Counter {
    fn increment(&self) {
        *self.count.borrow_mut() += 1;
    }

    fn get_count(&self) -> i32 {
        *self.count.borrow()
    }
}

fn main() {
    let counter = Counter {
        count: RefCell::new(0),
    };

    counter.increment();
    counter.increment();

    println!("Counter value: {}", counter.get_count());
}
