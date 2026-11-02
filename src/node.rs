pub struct Node {
    value : i32,
    left_child : Option<Box<Node>>,
    right_child : Option<Box<Node>>
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node { value: val, left_child: None, right_child: None }
    }
}