struct XorShift(u64);

impl XorShift {
    fn new() -> Self {
        // 適当な初期シード
        XorShift(88172645463325252)
    }
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

struct Node<T> {
    key: T,
    priority: u64,
    size: usize,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(key: T, priority: u64) -> Self {
        Node {
            key,
            priority,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn update(&mut self) {
        self.size = 1 + size_of(&self.left) + size_of(&self.right);
    }
}

fn size_of<T>(node: &Option<Box<Node<T>>>) -> usize {
    node.as_ref().map_or(0, |n| n.size)
}

fn merge<T>(l: Option<Box<Node<T>>>, r: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    match (l, r) {
        (Some(mut left), Some(mut right)) => {
            if left.priority > right.priority {
                left.right = merge(left.right.take(), Some(right));
                left.update();
                Some(left)
            } else {
                right.left = merge(Some(left), right.left.take());
                right.update();
                Some(right)
            }
        }
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
    }
}

fn split<T: Ord>(
    node: Option<Box<Node<T>>>,
    key: &T,
) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
    if let Some(mut n) = node {
        if n.key < *key {
            let (l, r) = split(n.right.take(), key);
            n.right = l;
            n.update();
            (Some(n), r)
        } else {
            let (l, r) = split(n.left.take(), key);
            n.left = r;
            n.update();
            (l, Some(n))
        }
    } else {
        (None, None)
    }
}

fn erase_node<T: Ord>(node: Option<Box<Node<T>>>, key: &T) -> Option<Box<Node<T>>> {
    let mut n = node?;
    if n.key == *key {
        return merge(n.left.take(), n.right.take());
    } else if n.key < *key {
        n.right = erase_node(n.right.take(), key);
    } else {
        n.left = erase_node(n.left.take(), key);
    }
    n.update();
    Some(n)
}

fn erase_max_node<T: Ord>(node: Option<Box<Node<T>>>) -> (Option<Box<Node<T>>>, Option<T>) {
    let Some(mut n) = node else {
        return (None, None);
    };
    if n.right.is_none() {
        return (n.left.take(), Some(n.key));
    }
    let (r, val) = erase_max_node(n.right.take());
    n.right = r;
    n.update();
    (Some(n), val)
}

fn erase_min_node<T>(node: Option<Box<Node<T>>>) -> (Option<Box<Node<T>>>, Option<T>) {
    let Some(mut n) = node else {
        return (None, None);
    };
    if n.left.is_none() {
        return (n.right.take(), Some(n.key));
    }
    let (l, val) = erase_min_node(n.left.take());
    n.left = l;
    n.update();
    (Some(n), val)
}

fn erase_nth_node<T>(node: Option<Box<Node<T>>>, k: usize) -> (Option<Box<Node<T>>>, Option<T>) {
    let Some(mut n) = node else {
        return (None, None);
    };
    let l_size = size_of(&n.left);
    if k == l_size {
        (merge(n.left.take(), n.right.take()), Some(n.key))
    } else if k < l_size {
        let (l, val) = erase_nth_node(n.left.take(), k);
        n.left = l;
        n.update();
        (Some(n), val)
    } else {
        let (r, val) = erase_nth_node(n.right.take(), k - l_size - 1);
        n.right = r;
        n.update();
        (Some(n), val)
    }
}

pub struct Treap<T> {
    root: Option<Box<Node<T>>>,
    rng: XorShift,
}

impl<T: Ord> Treap<T> {
    pub fn new() -> Self {
        Treap {
            root: None,
            rng: XorShift::new(),
        }
    }

    /// 追加: 要素を挿入します $O(\log N)$
    pub fn insert(&mut self, key: T) {
        let priority = self.rng.next();
        let new_node = Some(Box::new(Node::new(key, priority)));
        let root = self.root.take();
        let (l, r) = split(root, &new_node.as_ref().unwrap().key);
        self.root = merge(merge(l, new_node), r);
    }

    /// 検索: 要素が存在するか確認します $O(\log N)$
    pub fn contains(&self, key: &T) -> bool {
        let mut current = &self.root;
        while let Some(n) = current {
            if n.key == *key {
                return true;
            } else if n.key < *key {
                current = &n.right;
            } else {
                current = &n.left;
            }
        }
        false
    }

    /// 削除: 指定した要素を1つ削除します（存在すれば true） $O(\log N)$
    pub fn remove(&mut self, key: &T) -> bool {
        if !self.contains(key) {
            return false;
        }
        self.root = erase_node(self.root.take(), key);
        true
    }

    /// 最大値削除: 最大値を取得し、木から削除します $O(\log N)$
    pub fn pop_max(&mut self) -> Option<T> {
        let (new_root, val) = erase_max_node(self.root.take());
        self.root = new_root;
        val
    }

    /// 最小値削除: 最小値を取得し、木から削除します $O(\log N)$
    pub fn pop_min(&mut self) -> Option<T> {
        let (new_root, val) = erase_min_node(self.root.take());
        self.root = new_root;
        val
    }

    /// k番目削除: 小さい方からk番目（0-indexed）の要素を削除して返します $O(\log N)$
    pub fn remove_nth(&mut self, k: usize) -> Option<T> {
        if k >= self.len() {
            return None;
        }
        let (new_root, val) = erase_nth_node(self.root.take(), k);
        self.root = new_root;
        val
    }

    /// k番目取得: 小さい方からk番目（0-indexed）の要素への参照を返します $O(\log N)$
    pub fn nth(&self, mut k: usize) -> Option<&T> {
        let mut current = &self.root;
        while let Some(n) = current {
            let l_size = size_of(&n.left);
            if k == l_size {
                return Some(&n.key);
            } else if k < l_size {
                current = &n.left;
            } else {
                k -= l_size + 1;
                current = &n.right;
            }
        }
        None
    }

    /// 順位取得: 指定した要素より「真に小さい」要素の数を返します $O(\log N)$
    pub fn rank(&self, key: &T) -> usize {
        let mut count = 0;
        let mut current = &self.root;
        while let Some(n) = current {
            if n.key < *key {
                count += size_of(&n.left) + 1;
                current = &n.right;
            } else {
                current = &n.left;
            }
        }
        count
    }

    /// 下界: key以上の最小要素への参照を返します（Rustの `range(key..).next()` /
    /// C++の `lower_bound` に相当） $O(\log N)$
    pub fn lower_bound(&self, key: &T) -> Option<&T> {
        let mut current = &self.root;
        let mut candidate = None;
        while let Some(n) = current {
            if n.key < *key {
                current = &n.right;
            } else {
                candidate = Some(&n.key);
                current = &n.left;
            }
        }
        candidate
    }

    /// 上界: keyより真に大きい最小要素への参照を返します（Rustの
    /// `range((Excluded(key))..).next()` / C++の `upper_bound` に相当） $O(\log N)$
    pub fn upper_bound(&self, key: &T) -> Option<&T> {
        let mut current = &self.root;
        let mut candidate = None;
        while let Some(n) = current {
            if n.key <= *key {
                current = &n.right;
            } else {
                candidate = Some(&n.key);
                current = &n.left;
            }
        }
        candidate
    }

    /// key以下の最大要素への参照を返します（Rustの `range(..=key).next_back()` に相当） $O(\log N)$
    pub fn floor(&self, key: &T) -> Option<&T> {
        let mut current = &self.root;
        let mut candidate = None;
        while let Some(n) = current {
            if n.key <= *key {
                candidate = Some(&n.key);
                current = &n.right;
            } else {
                current = &n.left;
            }
        }
        candidate
    }

    /// 現在の要素数を返します
    pub fn len(&self) -> usize {
        size_of(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
