// Box使用场景：
// 1. 当需要在堆上分配数据时
// 2. 处理大型数据结构，避免栈溢出
// 3. 创建递归数据类型（如链表）
// 4. 当需要拥有固定大小类型时
//
// 实际应用场景：
// 1. 处理大型矩阵或图像数据
//    例如：let large_matrix = Box::new([[0.0; 1000]; 1000]);
// 2. 实现复杂的数据结构
//    例如：树结构、图结构的节点实现
// 3. 特征对象：将trait对象装箱以实现多态
//    例如：let handler: Box<dyn EventHandler> = Box::new(CustomHandler::new());
#[derive(Debug)]
struct List {
    head: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let list = List {
        head: Some(Box::new(Node {
            value: 1,
            next: Some(Box::new(Node {
                value: 2,
                next: None,
            })),
        })),
    };

    println!("List: {:?}", list);
    list.print_values();
}

impl List {
    fn print_values(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("Value: {}", node.value);
            current = &node.next;
        }
    }
}
