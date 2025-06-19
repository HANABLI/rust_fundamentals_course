struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn display(&self) {
        println!("Value: {}", self.value);
        if let Some(left) = &self.left {
            println!("LEFT");
            left.display();
        }
        if let Some(right) = &self.right {
            println!("RIGHT");
            right.display();
        }
        println!("UP");
    }
}

fn main() {
    let root = Node {
        value: 2,
        left: Some(Box::new(Node {
            value: 3,
            left: Some(Box::new(Node {
                value: 7,
                left: None,
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        })),
        right: None,
    };
    root.display();
}
