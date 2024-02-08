#![allow(dead_code)]

use std::fmt::Display;

const MAX_CHILDREN: usize = 5;
const INIT: Option<Box<Node>> = None;

#[derive(Debug)]
pub struct Node {
    // the number of keys
    pub n: usize,
    pub keys: [usize; MAX_CHILDREN - 1],
    pub children: [Option<Box<Node>>; MAX_CHILDREN],
    pub is_leaf: bool,
}

struct NodeFormatConfig<'a> {
    level: usize,
    right_most_node: &'a Node,
    first_child_found: bool,
}

impl Node {
    pub fn new() -> Self {
        Self {
            n: 0,
            keys: [0; MAX_CHILDREN - 1],
            children: [INIT; MAX_CHILDREN],
            is_leaf: true,
        }
    }

    pub fn new_boxed() -> Box<Self> {
        Box::new(Self {
            n: 0,
            keys: [0; MAX_CHILDREN - 1],
            children: [INIT; MAX_CHILDREN],
            is_leaf: true,
        })
    }

    fn is_node_full(&self) -> bool {
        self.n == MAX_CHILDREN - 1
    }

    // insert a key to the tree:
    //  if node is leaf
    //    if the leaf is full, split the leaf
    //      insert the key to the new parent
    //      return the new parent
    //    if the leaf is not full, insert the key (basic case)
    //  if node is internal
    //    insert the key to the child(recursive case)(go down)
    //    if have a new parent, then
    //      if the current node is full, split the node and return the new parent(go up)
    //      if the current node is not full, insert the new parent to the current node(DONE)
    //    if have no new parent, return None(DONE)
    //
    // split the node(left child):
    //   crete two new nodes, one for parent, one for right child
    //   copy the right half of the keys to the right child
    //   copy the right half of the children to the right child
    //   set the parent's first key to the middle key
    //   set the parent's first child to the left child
    //   set the parent's second child to the right child
    //   return the parent
    #[must_use]
    pub fn insert(&mut self, key: usize) -> Box<Node> {
        let n = match self.insert_down_to_leaf(key) {
            Some(nn) => nn,
            None => Box::new(std::mem::replace(self, Node::new())),
        };
        #[cfg(feature="debug2")]
        println!("insert DONE\n{}", n);
        n
    }

    #[must_use]
    fn insert_down_to_leaf(&mut self, key: usize) -> Option<Box<Node>> {
        if self.is_leaf {
            if self.is_node_full() {
                let mut new_parent = self.split_node();
                new_parent = new_parent.insert(key);
                #[cfg(feature="debug2")]
                println!("new_parent:\n{}", new_parent);
                return Some(new_parent);
            }
            // Else not full, insert directly
            let i = self.find_pos(key);
            if i < self.n {
                let mut k = self.n;
                while k > i {
                    self.keys[k] = self.keys[k - 1];
                    k -= 1;
                }
            }
            self.keys[i] = key;
            self.n += 1;
            #[cfg(feature="debug2")]
            println!("leaf inserted\n{}", self);
            return None;
        }
        // Else an internal node
        let i = self.find_pos(key);
        let child = self.children[i].as_mut().unwrap();
        let nn = child.insert_down_to_leaf(key);
        if let Some(mut nn) = nn {
            if self.is_node_full() {
                let mut new_parent = self.split_node();
                let children = &mut new_parent.children;

                if nn.keys[0] > new_parent.keys[0] {
                    let rc = children[1].as_mut().unwrap();
                    let i = rc.find_pos(nn.keys[0]);
                    let mut k = self.n;
                    while k > i {
                        rc.keys[k] = rc.keys[k - 1];
                    }
                    k = self.n + 1;
                    while k > i + 1 {
                        rc.children[k] = rc.children[k - 1].take();
                    }
                    rc.keys[i] = nn.keys[0];
                    rc.children[i] = nn.children[0].take();
                    rc.children[i + 1] = nn.children[1].take();
                    rc.n += 1;
                } else {
                    let lc = children[0].as_mut().unwrap();
                    let i = lc.find_pos(nn.keys[0]);
                    let mut k = self.n;
                    while k > i {
                        lc.keys[k] = lc.keys[k - 1];
                    }
                    k = self.n + 1;
                    while k > i + 1 {
                        lc.children[k] = lc.children[k - 1].take();
                    }
                    lc.keys[i] = nn.keys[0];
                    lc.children[i] = nn.children[0].take();
                    lc.children[i + 1] = nn.children[1].take();
                    lc.n += 1;
                }

                #[cfg(feature="debug2")]
                println!("internal(full) inserted\n{}", new_parent);
                return Some(new_parent);
            } else {
                let key = nn.keys[0];
                let i = self.find_pos(key);
                let mut k = self.n; // 1
                while k > i {
                    self.keys[k] = self.keys[k - 1];
                    k -= 1;
                }
                k = self.n + 1;
                while k > i + 1 {
                    self.children[k] = self.children[k - 1].take();
                    k -= 1;
                }
                self.keys[i] = key;
                self.children[i] = nn.children[0].take();
                self.children[i + 1] = nn.children[1].take();
                self.n += 1;
                #[cfg(feature="debug2")]
                println!("internal inserted\n{}", self);
                return None;
            }
        }
        None
    }

    #[must_use]
    fn split_node(&mut self) -> Box<Node> {
        let mut new_parent = Node::new_boxed();
        let mut right_child = Node::new_boxed();

        for i in 0..(self.n / 2 - 1) {
            right_child.keys[i] = self.keys[self.n / 2 + 1 + i];
            right_child.children[i] = self.children[self.n / 2 + 1 + i].take();
        }
        right_child.n = self.n / 2 - 1;
        right_child.is_leaf = self.is_leaf;

        new_parent.is_leaf = false;
        new_parent.keys[0] = self.keys[self.n / 2];
        new_parent.n = 1;
        self.n = self.n / 2;
        // `std::mem::replace` is an VERY IMPORTANT API for this case
        // Without it, I can not turn `self` to Box<Node>
        new_parent.children[0] = Some(Box::new(std::mem::replace(self, Node::new())));
        new_parent.children[1] = Some(right_child);

        new_parent
    }

    fn is_new_node(&self, node: &Node) -> bool {
        (self as *const Node) != (node as *const Node)
    }

    fn find_pos(&self, key: usize) -> usize {
        let mut i = 0;
        while i < self.n && key > self.keys[i] {
            i += 1;
        }
        i
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

    fn is_balanced(&self) -> bool {
        if self.is_leaf {
            return true;
        } else {
            let mut ph = 0;
            for child in &self.children {
                match child {
                    Some(bc) => {
                        let h = bc.height();
                        if ph == 0 {
                            ph = h;
                        }
                        if ph != h {
                            return false;
                        }
                    }
                    None => break,
                }
            }
        }
        true
    }

    fn height(&self) -> usize {
        if self.is_leaf {
            return 1;
        } else {
            let mut mh = 1;
            for child in &self.children {
                match child {
                    Some(bc) => {
                        mh = std::cmp::max(mh, bc.height());
                    }
                    None => break,
                }
            }
            return mh + 1;
        }
    }

    fn have_child(&self) -> bool {
        for child in &self.children {
            match child {
                Some(_) => return true,
                None => break,
            }
        }
        false
    }

    // see `build_tree`
    fn fmt_internal(
        &self,
        cfg: &mut NodeFormatConfig,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if !self.have_child() {
            if !cfg.first_child_found {
                cfg.first_child_found = true;
                writeln!(f)?;
            }
            for _ in 0..cfg.level {
                write!(f, " ")?;
            }
            write!(f, "[")?;
            for i in 0..self.n {
                write!(f, "{}", self.keys[i])?;
                if i < self.n - 1 {
                    write!(f, ", ")?;
                }
            }
            return writeln!(f, "],");
        }

        if !cfg.first_child_found {
            write!(f, "{{")?;
        } else {
            for _ in 0..cfg.level {
                write!(f, " ")?;
            }
            writeln!(f, "{{")?;
        }

        for i in 0..self.n {
            match self.children[i].as_ref() {
                Some(bc) => {
                    cfg.level += 1;
                    bc.fmt_internal(cfg, f)?;
                    cfg.level -= 1;
                }
                None => (),
            }
            for _ in 0..cfg.level {
                write!(f, " ")?;
            }
            writeln!(f, "{},", self.keys[i])?;
        }
        match self.children[self.n].as_ref() {
            Some(bc) => {
                cfg.level += 1;
                bc.fmt_internal(cfg, f)?;
                cfg.level -= 1;
                for _ in 0..cfg.level {
                    write!(f, " ")?;
                }
                if bc.as_ref() as *const Node == cfg.right_most_node as *const Node {
                    write!(f, "}}")?;
                } else if cfg.level == 0 {
                    write!(f, "}}")?;
                } else {
                    writeln!(f, "}},")?;
                }
            }
            None => (),
        }
        Ok(())
    }

    pub fn get_rightmost_node(&self) -> &Node {
        if self.is_leaf {
            return self;
        }
        for child in self.children.iter().rev() {
            match child {
                Some(bc) => return bc.get_rightmost_node(),
                None => continue,
            }
        }
        unreachable!()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut config = NodeFormatConfig {
            level: 0,
            right_most_node: self.get_rightmost_node(),
            first_child_found: false,
        };
        self.fmt_internal(&mut config, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // see page 9 of https://infolab.usc.edu/csci585/Spring2010/den_ar/indexing.pdf
    // output format:
    //   {{
    //     [1, 2, 3, 4],
    //    5,
    //     [6, 7],
    //    8,
    //     [9, 10],
    //    },
    //   11,
    //    {
    //     [12, 13, 14, 15],
    //    16,
    //     [17, 18, 19, 20],
    //    21,
    //     [22, 23, 24, 25],
    //    }}
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

    #[test]
    fn test_insert1() {
        let mut root = Node::new_boxed();
        root = root.insert(5);
        root = root.insert(8);
        root = root.insert(11);
        root = root.insert(16);
        assert_eq!(root.height(), 1);
        root = root.insert(21);
        assert_eq!(root.height(), 2);
        assert!(root.is_balanced());
        root = root.insert(1);
        root = root.insert(2);
        root = root.insert(6);
        root = root.insert(7);
        root = root.insert(9);
        root = root.insert(10);
        root = root.insert(12);
        root = root.insert(13);
        root = root.insert(17);
        root = root.insert(18);
        root = root.insert(22);
        root = root.insert(23);
        root = root.insert(3);
        root = root.insert(4);
        root = root.insert(14);
        root = root.insert(15);
        root = root.insert(19);
        root = root.insert(20);
        root = root.insert(24);
        root = root.insert(25);
        assert_eq!(root.height(), 3);
        assert!(root.is_balanced());
    }

    #[test]
    fn test_insert2() {
        let mut root = Node::new_boxed();
        root = root.insert(1);
        root = root.insert(2);
        root = root.insert(3);
        root = root.insert(4);
        root = root.insert(5);
        root = root.insert(6);
        root = root.insert(7);
        root = root.insert(8);
        root = root.insert(9);
        root = root.insert(10);
        root = root.insert(11);
        root = root.insert(12);
        root = root.insert(13);
        root = root.insert(14);
        root = root.insert(15);
        root = root.insert(16);
        root = root.insert(17);
        assert_eq!(root.height(), 3);
        assert!(root.is_balanced());
        let ans = format!("{}", root);
        let exp = r#"
{{
  [1, 2],
 3,
  [4, 5],
 6,
  [7, 8],
 },
9,
 {
  [10, 11],
 12,
  [13, 14],
 15,
  [16, 17],
 }}
"#;
        assert_eq!(ans, exp.trim());
    }

    #[test]
    fn test_format() {
        let root = build_tree();
        let ans = format!("{}", root);
        let exp = r#"
{{
  [1, 2, 3, 4],
 5,
  [6, 7],
 8,
  [9, 10],
 },
11,
 {
  [12, 13, 14, 15],
 16,
  [17, 18, 19, 20],
 21,
  [22, 23, 24, 25],
 }}"#;
        assert_eq!(ans, exp.trim_start());
    }
}
