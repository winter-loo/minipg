const MAX_KEYS: usize = 8;

#[derive(Debug)]
struct Node {
    n: usize,
    keys: [usize; MAX_KEYS],
    children: Vec<Node>,
    is_leaf: bool,
}

impl Node {
    fn new() -> Node {
        Node {
            n: 0,
            keys: [0; MAX_KEYS],
            children: Vec::new(),
            is_leaf: false,
        }
    }

    fn find(&self, key: usize) -> Option<&Node> {
        let mut i = 0;
        while i < self.n && key > self.keys[i] {
            i += 1;
        }
        if i < self.n && key == self.keys[i] {
            return Some(&self.children[i]);
        }
        if self.is_leaf {
            return None;
        }
        self.children[i].find(key)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let mut root = Node::new();
        root.keys[0] = 10;
        root.children.push(Node::new());
        assert_eq!(root.find(20).unwrap().n, 20);
    }
}

fn main() {
    let mut root = Node::new(10);
    root.children.push(Node::new(20));
    println!("{:?}", root);
}
