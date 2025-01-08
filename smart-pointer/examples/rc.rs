// Rc使用场景：
// 1. 在单线程环境下共享数据所有权
// 2. 实现多个所有者共享数据
// 3. 适用于需要引用计数的数据结构（如图或树）
//
// 实际应用场景：
// 1. GUI应用中的组件树：子组件和父组件共享数据
//    例如：let shared_state = Rc::new(AppState::new());
// 2. 缓存资源：多个对象共享同一份数据
//    例如：let shared_resource = Rc::new(Resource::load("data.txt"));
// 3. 图数据结构：节点被多个其他节点引用
//    例如：实现有向无环图(DAG)，其中节点可能有多个父节点
use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn main() {
    let node3 = Rc::new(Node {
        value: 3,
        next: None,
    });
    let node2 = Rc::new(Node {
        value: 2,
        next: Some(Rc::clone(&node3)),
    });
    let node1 = Rc::new(Node {
        value: 1,
        next: Some(Rc::clone(&node2)),
    });

    // println!("Node1 value: {}", node1.value);
    // if let Some(next_node) = &node1.next {
    //     println!("Node2 value: {}", next_node.value);
    //     if let Some(next_next_node) = &next_node.next {
    //         println!("Node3 value: {}", next_next_node.value);
    //     } else {
    //         println!("Node3 does not exist.");
    //     }
    // } else {
    //     println!("Node2 does not exist.");
    // }

    println!("Node1 value: {}", node1.value);
    match &node1.next {
        Some(next_node) => {
            println!("Node2 value: {}", next_node.value);
            match &next_node.next {
                Some(next_next_node) => {
                    println!("Node3 value: {}", next_next_node.value);
                }
                None => {
                    println!("Node3 does not exist.");
                }
            }
        }
        None => {
            println!("Node2 does not exist.");
        }
    }
}
