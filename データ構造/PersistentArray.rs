use std::rc::Rc;

#[derive(Clone)]
enum Node {
    Leaf(i64),
    Internal(Rc<Node>, Rc<Node>),
}

#[derive(Clone)]
struct PersistentArray {
    root: Rc<Node>,
    size: usize,
}
impl PersistentArray {
    pub fn new(size: usize, default: i64) -> Self {
        fn build(size: usize, default: i64) -> Rc<Node> {
            if size == 1 {
                Rc::new(Node::Leaf(default))
            } else {
                let half = size / 2;
                Rc::new(Node::Internal(build(half, default), build(half, default)))
            }
        }

        // 次の2冪サイズに拡張（管理を簡単にするため）
        let pow2_size = size.next_power_of_two();
        Self {
            root: build(pow2_size, default),
            size: pow2_size,
        }
    }

    pub fn get(&self, index: usize) -> i64 {
        fn get_rec(node: &Node, l: usize, r: usize, idx: usize) -> i64 {
            match node {
                Node::Leaf(val) => *val,
                Node::Internal(left, right) => {
                    let mid = (l + r) / 2;
                    if idx < mid {
                        get_rec(&left, l, mid, idx)
                    } else {
                        get_rec(&right, mid, r, idx)
                    }
                }
            }
        }
        assert!(index < self.size);
        get_rec(&self.root, 0, self.size, index)
    }

    pub fn update(&self, index: usize, value: i64) -> Self {
        fn update_rec(node: &Node, l: usize, r: usize, idx: usize, value: i64) -> Rc<Node> {
            if r - l == 1 {
                return Rc::new(Node::Leaf(value));
            }
            match node {
                Node::Internal(left, right) => {
                    let mid = (l + r) / 2;
                    if idx < mid {
                        Rc::new(Node::Internal(
                            update_rec(&left, l, mid, idx, value),
                            right.clone(),
                        ))
                    } else {
                        Rc::new(Node::Internal(
                            left.clone(),
                            update_rec(&right, mid, r, idx, value),
                        ))
                    }
                }
                Node::Leaf(_) => unreachable!(),
            }
        }

        assert!(index < self.size);
        PersistentArray {
            root: update_rec(&self.root, 0, self.size, index, value),
            size: self.size,
        }
    }
}
