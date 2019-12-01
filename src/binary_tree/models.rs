pub struct Node {
    pub value: i8,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn has_left(&self) -> bool {
        match &self.left {
            Some(_node) => true,
            None => false,
        }
    }

    pub fn left_value(&self) -> i8 {
        match &self.left {
            Some(_node) => _node.value,
            None => 0,
        }
    }

    pub fn has_right(&self) -> bool {
        match &self.right {
            Some(_node) => {
                println!("right value: {}", _node.value);
                true
            }
            None => false,
        }
    }
}
