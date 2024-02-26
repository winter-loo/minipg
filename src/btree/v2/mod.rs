#![allow(dead_code)]

use std::fmt::Display;

const MAX_CHILDREN: usize = 5;
const INIT: Option<Box<Node>> = None;

#[derive(Debug)]
pub struct Node {
    // the number of keys
    pub n: usize,
    // one more for hypotetical right child
    // The actual maximum number of keys is `MAX_CHILDREN - 1`
    pub keys: [usize; MAX_CHILDREN],
    // The actual maximum number of child is `MAX_CHILDREN`
    pub children: [Option<Box<Node>>; MAX_CHILDREN + 1],
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
            keys: [0; MAX_CHILDREN],
            children: [INIT; MAX_CHILDREN + 1],
            is_leaf: true,
        }
    }

    pub fn new_boxed() -> Box<Self> {
        Box::new(Self {
            n: 0,
            keys: [0; MAX_CHILDREN],
            children: [INIT; MAX_CHILDREN + 1],
            is_leaf: true,
        })
    }

    fn is_node_full(&self) -> bool {
        self.n == MAX_CHILDREN - 1
    }

    // insert a key to the tree:
    //  if node is leaf(base case)
    //    insert the key to the node
    //    if the node need split, split the node
    //  if node is internal
    //    insert the key to the child(recursive case)(go down)
    //    if the child is not splited, DONE
    //    if the child is splited,
    //      insert the child key to the current node
    //      insert the left child of child node to the current node
    //      insert the right child of child node to the current node
    //      split the current node
    //
    // split the node(left child):
    //   crete two new nodes, one for parent, one for right child
    //   copy the right half of the keys to the right child
    //   copy the right half of the children to the right child
    //   set the parent's first key to the middle key
    //   set the parent's first child to the left child
    //   set the parent's second child to the right child
    //   return the parent
    pub fn insert(&mut self, key: usize) {
        self.insert_down_to_leaf(key);
    }

    fn need_split(&self) -> bool {
        self.n >= MAX_CHILDREN
    }

    fn insert_key(&mut self, key: usize, index: usize) {
        let i = if index == usize::MAX {
            self.find_pos(key)
        } else {
            index
        };
        let mut k = self.n;
        assert!(k < MAX_CHILDREN);
        while k > i {
            self.keys[k] = self.keys[k - 1];
            k -= 1;
        }
        self.keys[i] = key;
        self.n += 1;
    }

    fn insert_child(&mut self, index: usize, lc: Option<Box<Node>>, rc: Option<Box<Node>>) {
        let i = index;
        let mut k = MAX_CHILDREN;
        while k > (i + 1) {
            self.children[k] = self.children[k - 1].take();
            k -= 1;
        }
        self.children[i] = lc;
        self.children[i + 1] = rc;
    }

    fn insert_down_to_leaf(&mut self, key: usize) -> bool {
        if self.is_leaf {
            self.insert_key(key, usize::MAX);
            if self.need_split() {
                self.split_node();
                return true;
            }
            return false;
        } else {
            let i = self.find_pos(key);
            let child = self.children[i].as_mut().unwrap();
            let splited = child.insert_down_to_leaf(key);
            if splited {
                let key = child.keys[0];
                let lc = child.children[0].take();
                let rc = child.children[1].take();
                self.insert_key(key, i);
                self.insert_child(i, lc, rc);

                if self.need_split() {
                    self.split_node();

                    #[cfg(feature="debug2")]
                    println!("internal(full) inserted\n{}", self);
                    return true;
                } else {
                    #[cfg(feature="debug2")]
                    println!("internal inserted\n{}", self);
                    return false;
                }
            }
        }
        return false;
    }

    fn split_node(&mut self) {
        let mut new_parent = Node::new_boxed();
        let mut right_child = Node::new_boxed();

        for i in 0..((self.n - 1) / 2) {
            right_child.keys[i] = self.keys[self.n / 2 + 1 + i];
            right_child.children[i] = self.children[self.n / 2 + 1 + i].take();
        }
        right_child.children[(self.n - 1) / 2] = self.children[self.n].take();
        right_child.n = (self.n - 1) / 2;
        right_child.is_leaf = self.is_leaf;

        new_parent.is_leaf = false;
        new_parent.keys[0] = self.keys[self.n / 2];
        new_parent.n = 1;
        self.n = (self.n - 1) / 2;
        // `std::mem::replace` is an VERY IMPORTANT API for this case
        // Without it, I can not turn `self` to Box<Node>
        new_parent.children[0] = Some(Box::new(std::mem::replace(self, Node::new())));
        new_parent.children[1] = Some(right_child);

        *self = *new_parent;
    }

    fn is_new_node(&self, node: &Node) -> bool {
        (self as *const Node) != (node as *const Node)
    }

    fn find_pos(&self, key: usize) -> usize {
        let mut i = 0;
        while i < self.n && i < (MAX_CHILDREN - 1) && key > self.keys[i] {
            i += 1;
        }
        i
    }

    fn find(&self, key: usize) -> Option<&Node> {
        let i = self.find_pos(key);
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

    fn fill_child(&mut self, i: usize) {
        if i > 0 {
            if let Some(left) = self.children[i - 1].as_mut() {
                if left.n > (MAX_CHILDREN - 1) / 2 {
                    self.borrow_from_left(i);
                    return;
                }
            }
        }
        if i < self.n {
            if let Some(right) = self.children[i + 1].as_mut() {
                if right.n > (MAX_CHILDREN - 1) / 2 {
                    self.borrow_from_right(i);
                    return;
                }
            }
        }
        if i > 0 {
            self.merge(i);
        } else {
            self.merge(i + 1);
        }
    }

    fn borrow_from_left(&mut self, i: usize) {
        let (left, child) = self.children.split_at_mut(i);
        let left = left[i - 1].as_mut().unwrap();
        let child = child[0].as_mut().unwrap();

        for j in (1..child.n + 1).rev() {
            child.keys[j] = child.keys[j - 1];
        }
        child.keys[0] = self.keys[i - 1];
        self.keys[i - 1] = left.keys[left.n - 1];
        if !left.is_leaf {
            for j in (1..child.n + 2).rev() {
                child.children[j] = child.children[j - 1].take();
            }
            child.children[0] = left.children[left.n].take();
        }
        child.n += 1;
        left.n -= 1;
    }

    fn borrow_from_right(&mut self, i: usize) {
        let (child, right) = self.children.split_at_mut(i);
        let child = child[i - 1].as_mut().unwrap();
        let right = right[0].as_mut().unwrap();
        
        child.keys[child.n] = self.keys[i];
        self.keys[i] = right.keys[0];
        for j in 0..right.n - 1 {
            right.keys[j] = right.keys[j + 1];
        }
        if !right.is_leaf {
            child.children[child.n + 1] = right.children[0].take();
            for j in 0..right.n {
                right.children[j] = right.children[j + 1].take();
            }
        }
        child.n += 1;
        right.n -= 1;
    }

    fn merge(&mut self, i: usize) {
        let (child, right) = self.children.split_at_mut(i);
        let child = child[i - 1].as_mut().unwrap();
        let right = right[0].as_mut().unwrap();

        for j in 0..right.n {
            right.keys[child.n + 1 + j] = right.keys[j];
        }
        right.keys[child.n] = self.keys[i-1];
        for j in 0..child.n {
            right.keys[j] = child.keys[j];
        }
        right.n += child.n + 1;
        if !right.is_leaf {
            for j in 0..=child.n {
                right.children[child.n + 1 + j] = right.children[j].take();
            }
        }
        if !child.is_leaf {
            for j in 0..=child.n {
                right.children[j] = child.children[j].take();
            }
        }
        for j in i..self.n {
            self.keys[j - 1] = self.keys[j];
        }
        for j in i..=self.n {
            self.children[j - 1] = self.children[j].take();
        }
        self.n -= 1;
    }

    fn predecessor(&self, i: usize) -> usize {
        let mut cur = self.children[i].as_ref().unwrap();
        while !cur.is_leaf {
            cur = cur.children[cur.n].as_ref().unwrap();
        }
        cur.keys[cur.n - 1]
    }

    fn successor(&self, i: usize) -> usize {
        let mut cur = self.children[i + 1].as_ref().unwrap();
        while !cur.is_leaf {
            cur = cur.children[0].as_ref().unwrap();
        }
        cur.keys[0]
    }

    fn delete_internal_node(&mut self, i: usize) {
        let key = self.keys[i];
        if self.children[i].as_ref().unwrap().n > (MAX_CHILDREN - 1) / 2 {
            if self.n >= MAX_CHILDREN {
                self.split_node();
                if i > MAX_CHILDREN / 2 {
                    let i = i - MAX_CHILDREN / 2 - 1;
                    self.children[1].as_mut().unwrap().delete_internal_node(i);
                } else {
                    self.children[0].as_mut().unwrap().delete_internal_node(0);
                }
            } else {
                let pred = self.predecessor(i);
                self.keys[i] = pred;
                self.children[i].as_mut().unwrap().delete(pred);
            }
        } else if self.children[i + 1].as_ref().unwrap().n > (MAX_CHILDREN - 1) / 2 {
            if self.n >= MAX_CHILDREN {
                self.split_node();
                if i > MAX_CHILDREN / 2 {
                    let i = i - MAX_CHILDREN / 2 - 1;
                    self.children[1].as_mut().unwrap().delete_internal_node(i);
                } else {
                    self.children[0].as_mut().unwrap().delete_internal_node(0);
                }
            } else {
                let succ = self.successor(i);
                self.keys[i] = succ;
                self.children[i + 1].as_mut().unwrap().delete(succ);
            }
        } else {
            self.merge(i + 1);
            if let Some(child) = self.children[i].as_mut() {
                child.delete(key);
            }
            if self.n == 0 {
                *self = *self.children[0].take().unwrap();
            }
        }
    }

    pub fn delete(&mut self, key: usize) {
        let i = self.find_pos(key);
        // if the key is found in the current node
        if i < self.n && key == self.keys[i] {
            if self.is_leaf {
                for j in i..self.n - 1 {
                    self.keys[j] = self.keys[j + 1];
                }
                self.n -= 1;
            } else {
                self.delete_internal_node(i);
            }
        } else {
            // if the key is not found in the current node
            if let Some(child) = self.children[i].as_ref() {
                let oldn = self.n;
                if child.n < 1 + (MAX_CHILDREN - 1) / 2 {
                    self.fill_child(i);
                }
                let merged = oldn != self.n;
                if i > 0 && merged {
                    self.children[i - 1].as_mut().unwrap().delete(key);
                } else {
                    self.children[i].as_mut().unwrap().delete(key);
                }
                if self.n == 0 {
                    *self = *self.children[0].take().unwrap();
                }
            }
        }
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
        root.insert(5);
        root.insert(8);
        root.insert(11);
        root.insert(16);
        assert_eq!(root.height(), 1);
        root.insert(21);
        assert_eq!(root.height(), 2);
        assert!(root.is_balanced());
        let input = [1, 2, 6, 7, 9, 10, 12, 13, 17, 18, 22, 23, 3, 4, 14, 15, 19, 20, 24, 25];
        for i in input {
            root.insert(i);
        }
        assert_eq!(root.height(), 3);
        assert!(root.is_balanced());
        println!("{}", root);
    }

    #[test]
    fn test_insert2() {
        let mut root = Node::new_boxed();
        let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        for i in input {
            root.insert(i);
        }
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

    #[test]
    fn test_delete_from_leaf() {
        let mut root = Node::new_boxed();
        let input = [11, 1, 2, 20, 21, 5, 7, 4, 8, 3];
        for i in input {
            root.insert(i);
        }
        assert_eq!(root.height(), 2);
        assert!(root.is_balanced());
        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2, 3],
4,
 [5, 7, 8],
11,
 [20, 21],
}"#;
        assert_eq!(ans, exp.trim());

        // delete directly
        root.delete(8);

        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2, 3],
4,
 [5, 7],
11,
 [20, 21],
}
"#;
        assert_eq!(ans, exp.trim());

        // borrow a key
        root.delete(5);

        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2],
3,
 [4, 7],
11,
 [20, 21],
}
"#;
        assert_eq!(ans, exp.trim());

        // merge two children
        root.delete(4);

        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2, 3, 7],
11,
 [20, 21],
}
"#;
        assert_eq!(ans, exp.trim());

        // borrow a key
        root.delete(21);

        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2, 3],
7,
 [11, 20],
}
"#;
        assert_eq!(ans, exp.trim());
    }

    #[test]
    fn test_delete_from_internal() {
        let mut root = Node::new_boxed();
        let input = [5, 8, 11, 16, 21, 1, 2, 6, 7, 9, 10, 12, 13, 17, 18, 22, 23, 19];
        for i in input {
            root.insert(i);
        }
        assert_eq!(root.height(), 3);
        assert!(root.is_balanced());
        let ans = format!("{}", root);
        let exp = r#"
{{
  [1, 2],
 5,
  [6, 7],
 8,
  [9, 10],
 },
11,
 {
  [12, 13],
 16,
  [17, 18, 19],
 21,
  [22, 23],
 }}
"#;
        assert_eq!(ans, exp.trim());

        root.delete(16);
        let ans = format!("{}", root);
        let exp = r#"
{{
  [1, 2],
 5,
  [6, 7],
 8,
  [9, 10],
 },
11,
 {
  [12, 13],
 17,
  [18, 19],
 21,
  [22, 23],
 }}
"#;
        assert_eq!(ans, exp.trim());

        root.delete(5);
        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2, 6, 7],
8,
 [9, 10],
11,
 [12, 13],
17,
 [18, 19],
21,
 [22, 23],
}
"#;
        assert_eq!(ans, exp.trim());
        assert_eq!(root.height(), 2);

        root.delete(8);
        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2, 6],
7,
 [9, 10],
11,
 [12, 13],
17,
 [18, 19],
21,
 [22, 23],
}
"#;
        assert_eq!(ans, exp.trim());

        root.delete(11);
        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2, 6],
7,
 [9, 10, 12, 13],
17,
 [18, 19],
21,
 [22, 23],
}
"#;
        assert_eq!(ans, exp.trim());

        root.delete(6);
        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2],
7,
 [9, 10, 12, 13],
17,
 [18, 19],
21,
 [22, 23],
}
"#;
        assert_eq!(ans, exp.trim());

        root.delete(17);
        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2],
7,
 [9, 10, 12],
13,
 [18, 19],
21,
 [22, 23],
}
"#;
        assert_eq!(ans, exp.trim());

        root.delete(21);
        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2],
7,
 [9, 10, 12],
13,
 [18, 19, 22, 23],
}
"#;
        assert_eq!(ans, exp.trim());

        root.delete(9);
        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2],
7,
 [10, 12],
13,
 [18, 19, 22, 23],
}
"#;
        assert_eq!(ans, exp.trim());

        root.delete(7);
        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2, 10, 12],
13,
 [18, 19, 22, 23],
}
"#;
        assert_eq!(ans, exp.trim());

        root.delete(10);
        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2, 12],
13,
 [18, 19, 22, 23],
}
"#;
        assert_eq!(ans, exp.trim());

        root.delete(12);
        let ans = format!("{}", root);
        let exp = r#"
{
 [1, 2],
13,
 [18, 19, 22, 23],
}
"#;
        assert_eq!(ans, exp.trim());
    }
}
