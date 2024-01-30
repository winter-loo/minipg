const MAX_CHILDREN: usize = 5;
const INIT: Option<Box<Node>> = None;

#[derive(Debug)]
struct Node {
    n: usize,
    keys: [usize; MAX_CHILDREN - 1],
    children: [Option<Box<Node>>; MAX_CHILDREN],
    is_leaf: bool,
}

impl Node {
    fn new() -> Node {
        Node {
            n: 0,
            keys: [0; MAX_CHILDREN - 1],
            children: [INIT; MAX_CHILDREN],
            is_leaf: false,
        }
    }

    fn find(&self, key: usize) -> Option<&Node> {
        let mut i = 0;
        while i < self.n && key > self.keys[i] {
            i += 1;
        }
        if i < self.n && key == self.keys[i] {
            return Some(self);
        }
        if self.is_leaf {
            return None;
        }
        self.children[i].as_ref().unwrap().find(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let mut root = Node::new();
        root.is_leaf = true;
        root.keys[0] = 10;
        root.n += 1;
        assert!(root.find(20).is_none());

        root.keys[1] = 20;
        root.n += 1;
        let it = root.find(20);
        assert!(it.is_some());
        assert_eq!(it.unwrap().keys[1], 20); 
    }
}

fn main() {
    let mut root = Node::new();
    root.keys[0] = 10;
    println!("{:?}", root);
}
