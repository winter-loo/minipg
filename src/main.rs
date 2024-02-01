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

    fn node_is_full(&self) -> bool {
        self.n == MAX_CHILDREN - 1
    }

    fn node_find_pos(&self, key: usize) -> usize {
        let mut i = 0;
        while i < self.n && key > self.keys[i] {
            i += 1;
        }
        i
    }

    fn insert(&mut self, key: usize) {
        self.insert_leaf(self, key);
    }

    fn split_node(node: Node) {}

    fn insert_internal(&mut self, n: Node) {
        if self.node_is_full() {
            split_node(n);
        } else {
            self.keys[i] = key;
            self.n += 1;
        }
    }

    fn insert_leaf(&mut self, key: usize) -> Option<Node> {
        let i = self.node_find_pos(key);
        if self.children[i].is_some() {
            let res = self.insert_leaf(&mut self.children[i].unwrap().deref(), key);
            match res {
                Some(n) => insert_internal(n),
                None => return None,
            }
        } else {
            if self.node_is_full() {
                let &mut parent = Node::new();
                parent.is_leaf = false;
                parent.n = 1;
                parent.keys[0] = self.keys[self.n / 2];
                let &mut child1 = Node::new();
                child1.is_leaf = true;
                child1.n = self.n / 2;
                for i in 0..child1.n {
                    child1.keys[i] = self.keys[i];
                }
                let &mut child2 = Noew::new();
                child2.is_leaf = true;
                child2.n = self.n / 2;
                for i in 0..child2.n {
                    child2.keys[i] = self.keys[1 + i + self.n / 2];
                }
                parent.children[0] = Some(Box::new(child1));
                parent.children[1] = Some(Box::new(child2));

                return Some(parent);
            } else {
                self.keys[i] = key;
                self.n += 1;
                return None;
            }
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

    // see page 9 of https://infolab.usc.edu/csci585/Spring2010/den_ar/indexing.pdf
    fn build_tree() -> Node {
        let mut root = Node::new();
        root.is_leaf = false;
        root.keys[0] = 11;
        root.n += 1;

        let mut leaf1 = Node::new();
        leaf1.is_leaf = true;
        leaf1.keys[0] = 1;
        leaf1.keys[1] = 2;
        leaf1.keys[2] = 3;
        leaf1.keys[3] = 4;
        leaf1.n = 4;

        let mut leaf2 = Node::new();
        leaf2.is_leaf = true;
        leaf2.keys[0] = 6;
        leaf2.keys[1] = 7;
        leaf2.n = 2;

        let mut leaf3 = Node::new();
        leaf3.is_leaf = true;
        leaf3.keys[0] = 9;
        leaf3.keys[1] = 10;
        leaf3.n = 2;

        let mut leaf4 = Node::new();
        leaf4.is_leaf = true;
        leaf4.keys[0] = 12;
        leaf4.keys[1] = 13;
        leaf4.keys[2] = 14;
        leaf4.keys[3] = 15;
        leaf4.n = 4;

        let mut leaf5 = Node::new();
        leaf5.is_leaf = true;
        leaf5.keys[0] = 17;
        leaf5.keys[1] = 18;
        leaf5.keys[2] = 19;
        leaf5.keys[3] = 20;
        leaf5.n = 4;

        let mut leaf6 = Node::new();
        leaf6.is_leaf = true;
        leaf6.keys[0] = 22;
        leaf6.keys[1] = 23;
        leaf6.keys[2] = 24;
        leaf6.keys[3] = 25;
        leaf6.n = 4;

        let mut inode1 = Node::new();
        inode1.is_leaf = false;
        inode1.keys[0] = 5;
        inode1.keys[1] = 8;
        inode1.n = 2;

        let mut inode2 = Node::new();
        inode2.is_leaf = false;
        inode2.keys[0] = 16;
        inode2.keys[1] = 21;
        inode2.n = 2;

        inode1.children[0] = Some(Box::new(leaf1));
        inode1.children[1] = Some(Box::new(leaf2));
        inode1.children[2] = Some(Box::new(leaf3));

        inode2.children[0] = Some(Box::new(leaf4));
        inode2.children[1] = Some(Box::new(leaf5));
        inode2.children[2] = Some(Box::new(leaf6));

        root.children[0] = Some(Box::new(inode1));
        root.children[1] = Some(Box::new(inode2));

        root
    }

    #[test]
    fn test_find() {
        let root = build_tree();

        let it = root.find(20);
        assert!(it.is_some());
        assert_eq!(it.unwrap().keys[3], 20);

        let it = root.find(100);
        assert!(it.is_none());
    }
}

fn main() {
    let mut root = Node::new();
    root.keys[0] = 10;
    println!("{:?}", root);
}
