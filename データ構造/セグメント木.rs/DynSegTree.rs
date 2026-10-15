// dynamic_segtree.rs
// ACL-style dynamic segment tree (sparse), generic over a Monoid trait.

use std::sync::Arc;

// ---------------- Monoid trait ----------------
pub trait Monoid {
    type S: Clone;
    fn identity() -> Self::S;
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
}

#[derive(Debug)]
struct Node<S: Clone> {
    val: S,
    left: Option<Box<Node<S>>>,
    right: Option<Box<Node<S>>>,
}

impl<S: Clone> Node<S> {
    fn new(val: S) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

/// Dynamic (implicit) segment tree that stores only visited nodes.
/// Works for index range [0, size).
pub struct DynSegTree<M: Monoid> {
    root: Option<Box<Node<M::S>>>,
    size: usize,
    _marker: std::marker::PhantomData<M>,
}

impl<M: Monoid> DynSegTree<M> {
    pub fn new(size: usize) -> Self {
        DynSegTree {
            root: None,
            size,
            _marker: std::marker::PhantomData,
        }
    }

    fn pull(node: &mut Box<Node<M::S>>) {
        let lval = node
            .left
            .as_ref()
            .map(|b| &b.val)
            .cloned()
            .unwrap_or_else(|| M::identity());
        let rval = node
            .right
            .as_ref()
            .map(|b| &b.val)
            .cloned()
            .unwrap_or_else(|| M::identity());
        node.val = M::binary_operation(&lval, &rval);
    }

    pub fn set(&mut self, pos: usize, value: M::S) {
        assert!(pos < self.size);
        self.root = Self::set_rec(self.root.take(), 0, self.size, pos, value);
    }

    fn set_rec(
        mut node: Option<Box<Node<M::S>>>,
        l: usize,
        r: usize,
        pos: usize,
        value: M::S,
    ) -> Option<Box<Node<M::S>>> {
        if node.is_none() {
            node = Some(Box::new(Node::new(M::identity())));
        }
        let mut n = node.unwrap();
        if r - l == 1 {
            n.val = value;
            return Some(n);
        }
        let m = (l + r) >> 1;
        if pos < m {
            n.left = Self::set_rec(n.left.take(), l, m, pos, value);
        } else {
            n.right = Self::set_rec(n.right.take(), m, r, pos, value);
        }
        Self::pull(&mut n);
        Some(n)
    }

    pub fn get(&self, pos: usize) -> M::S {
        assert!(pos < self.size);
        Self::get_rec(self.root.as_ref(), 0, self.size, pos)
    }

    fn get_rec(node: Option<&Box<Node<M::S>>>, l: usize, r: usize, pos: usize) -> M::S {
        if node.is_none() {
            return M::identity();
        }
        let n = node.unwrap();
        if r - l == 1 {
            return n.val.clone();
        }
        let m = (l + r) >> 1;
        if pos < m {
            Self::get_rec(n.left.as_ref(), l, m, pos)
        } else {
            Self::get_rec(n.right.as_ref(), m, r, pos)
        }
    }

    pub fn prod(&self, ql: usize, qr: usize) -> M::S {
        assert!(ql <= qr && qr <= self.size);
        Self::prod_rec(self.root.as_ref(), 0, self.size, ql, qr)
    }

    fn prod_rec(node: Option<&Box<Node<M::S>>>, l: usize, r: usize, ql: usize, qr: usize) -> M::S {
        if ql <= l && r <= qr {
            return node.map(|n| n.val.clone()).unwrap_or_else(|| M::identity());
        }
        if node.is_none() || qr <= l || r <= ql {
            return M::identity();
        }
        let n = node.unwrap();
        let m = (l + r) >> 1;
        let left_res = Self::prod_rec(n.left.as_ref(), l, m, ql, qr);
        let right_res = Self::prod_rec(n.right.as_ref(), m, r, ql, qr);
        M::binary_operation(&left_res, &right_res)
    }

    pub fn all_prod(&self) -> M::S {
        self.root
            .as_ref()
            .map(|n| n.val.clone())
            .unwrap_or_else(|| M::identity())
    }

    /// max_right: given `l` and predicate `f`, find max r such that f(prod(l,r)) is true.
    pub fn max_right<P>(&self, l: usize, pred: P) -> usize
    where
        P: Fn(&M::S) -> bool,
    {
        assert!(l <= self.size);
        if l == self.size {
            return self.size;
        }
        let mut acc = M::identity();
        Self::max_right_rec(&self.root, 0, self.size, l, &mut acc, &pred)
    }

    fn max_right_rec<P>(
        node: &Option<Box<Node<M::S>>>,
        nl: usize,
        nr: usize,
        ql: usize,
        acc: &mut M::S,
        pred: &P,
    ) -> usize
    where
        P: Fn(&M::S) -> bool,
    {
        if nr <= ql {
            return nr;
        }
        if node.is_none() {
            return nr;
        }
        if ql <= nl {
            let node_ref = node.as_ref().unwrap();
            let combined = M::binary_operation(acc, &node_ref.val);
            if pred(&combined) {
                *acc = combined;
                return nr;
            }
            if nr - nl == 1 {
                return nl;
            }
            let mid = (nl + nr) >> 1;
            let left = Self::max_right_rec(&node_ref.left, nl, mid, ql, acc, pred);
            if left < mid {
                return left;
            }
            Self::max_right_rec(&node_ref.right, mid, nr, ql, acc, pred)
        } else {
            let mid = (nl + nr) >> 1;
            let left = Self::max_right_rec(&node.as_ref().unwrap().left, nl, mid, ql, acc, pred);
            if left < mid {
                return left;
            }
            Self::max_right_rec(&node.as_ref().unwrap().right, mid, nr, ql, acc, pred)
        }
    }

    /// min_left: given `r` and predicate `f`, find min l such that f(prod(l,r)) is true.
    pub fn min_left<P>(&self, r: usize, pred: P) -> usize
    where
        P: Fn(&M::S) -> bool,
    {
        assert!(r <= self.size);
        if r == 0 {
            return 0;
        }
        let mut acc = M::identity();
        Self::min_left_rec(&self.root, 0, self.size, r, &mut acc, &pred)
    }

    fn min_left_rec<P>(
        node: &Option<Box<Node<M::S>>>,
        nl: usize,
        nr: usize,
        qr: usize,
        acc: &mut M::S,
        pred: &P,
    ) -> usize
    where
        P: Fn(&M::S) -> bool,
    {
        if qr <= nl {
            return nl;
        }
        if node.is_none() {
            return nl;
        }
        if nr <= qr {
            let node_ref = node.as_ref().unwrap();
            let combined = M::binary_operation(&node_ref.val, acc);
            if pred(&combined) {
                *acc = combined;
                return nl;
            }
            if nr - nl == 1 {
                return nr;
            }
            let mid = (nl + nr) >> 1;
            let right = Self::min_left_rec(&node_ref.right, mid, nr, qr, acc, pred);
            if right > mid {
                return right;
            }
            Self::min_left_rec(&node_ref.left, nl, mid, qr, acc, pred)
        } else {
            let mid = (nl + nr) >> 1;
            let right = Self::min_left_rec(&node.as_ref().unwrap().right, mid, nr, qr, acc, pred);
            if right > mid {
                return right;
            }
            Self::min_left_rec(&node.as_ref().unwrap().left, nl, mid, qr, acc, pred)
        }
    }
}

// ---------------- Example Monoid ----------------
// struct composite;
// impl Monoid for composite {
//     type S = (i64, i64); // f(x) = ax + b
//     fn identity() -> Self::S {
//         (1, 0)
//     }
//     fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
//         (a.0 * b.0, a.1 * b.0 + b.1)
//     }
// }

// fn main() {
//     let mut st = DynSegTree::<composite>::new(1 << 30);
//     st.set(3, (2, 1));
//     st.set(10, (3, 5));
//     println!("prod(0,20) = {:?}", st.prod(0,20));
//     let r = st.max_right(0, |s: &(i64,i64)| s.0 <= 6);
//     println!("max_right = {}", r);
// }
