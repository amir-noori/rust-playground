

pub enum NodeType<N> {
    ValueNode(N),
    EmptyNode,
}

struct Node<T> {
    value: Option<T>,
    next: NodeType<Box<Node<T>>>
}

struct LinkedList<T> {
    head: Node<T>,
    tail: Node<T>,
}

impl<T> LinkedList<T> {

    pub fn new(val: T) -> Self {
        LinkedList {
            head: Node {
                value: Some(val),
                next: NodeType::EmptyNode
            },
            tail: Node {
                value: None,
                next: NodeType::EmptyNode
            }
        }
    }

    fn get_next(&mut self) -> &Option<T> {
        let result = match &self.head.next {
            NodeType::ValueNode(n) => &n.value,
            NodeType::EmptyNode => &self.head.value
        };

        result
    }

    fn put(&mut self, t: T) {
        let node = Node {
            value: Some(t),
            next: NodeType::EmptyNode
        };
        self.tail.next = NodeType::ValueNode(Box::new(node));
        // self.tail = node;
    }
}


pub fn test() {

    let mut my_list = LinkedList::new(10);
    my_list.put(20);
    my_list.put(30);

    match my_list.get_next() {
        Some(t) => println!("first value: {}", t),
        None => println!("first value: none"),
    }

    match my_list.get_next() {
        Some(t) => println!("first value: {}", t),
        None => println!("first value: none"),
    }

}