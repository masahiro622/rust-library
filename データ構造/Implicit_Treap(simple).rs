/// シンプルなXorshift乱数生成器
struct XorShift {
    state: u64,
}

impl XorShift {
    fn new() -> Self {
        XorShift {
            state: 88172645463325252,
        }
    }
    fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
}

/// Implicit Treapのノード
/// ※値の大小で比較しないため、T: Ord の制約は不要です
struct Node<T> {
    value: T,
    priority: u64,
    size: usize, // 部分木の要素数を保持する
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, priority: u64) -> Self {
        Node {
            value,
            priority,
            size: 1, // 初期状態は自分自身の1のみ
            left: None,
            right: None,
        }
    }

    /// 左右の子のサイズを見て、自身のサイズを再計算する
    fn update(&mut self) {
        let l_size = self.left.as_ref().map_or(0, |n| n.size);
        let r_size = self.right.as_ref().map_or(0, |n| n.size);
        self.size = 1 + l_size + r_size;
    }
}

pub struct ImplicitTreap<T> {
    root: Option<Box<Node<T>>>,
    rng: XorShift,
}

impl<T> ImplicitTreap<T> {
    pub fn new() -> Self {
        ImplicitTreap {
            root: None,
            rng: XorShift::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |n| n.size)
    }

    /// 【Split】: 木を「先頭から k 個」と「それ以降」に分割する
    fn split_at(
        node: Option<Box<Node<T>>>,
        k: usize,
    ) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
        match node {
            None => (None, None),
            Some(mut n) => {
                let l_size = n.left.as_ref().map_or(0, |x| x.size);
                if k <= l_size {
                    // 左部分木の中に k 番目の境界がある場合
                    let (l, r) = Self::split_at(n.left.take(), k);
                    n.left = r;
                    n.update(); // 構造が変わったのでサイズを更新
                    (l, Some(n))
                } else {
                    // 右部分木の中に k 番目の境界がある場合
                    // 左部分木(l_size)と自分自身(+1)を引いた分を右部分木から切り出す
                    let (l, r) = Self::split_at(n.right.take(), k - l_size - 1);
                    n.right = l;
                    n.update(); // 構造が変わったのでサイズを更新
                    (Some(n), r)
                }
            }
        }
    }

    /// 【Merge】: 2つの木を結合する
    fn merge(left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        match (left, right) {
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.priority > r.priority {
                    l.right = Self::merge(l.right.take(), Some(r));
                    l.update(); // 構造が変わったのでサイズを更新
                    Some(l)
                } else {
                    r.left = Self::merge(Some(l), r.left.take());
                    r.update(); // 構造が変わったのでサイズを更新
                    Some(r)
                }
            }
        }
    }

    /// 先頭から k 番目に要素を挿入する (0-indexed)
    pub fn insert_at(&mut self, k: usize, value: T) {
        assert!(k <= self.len(), "Index out of bounds");
        let priority = self.rng.next();
        let new_node = Some(Box::new(Node::new(value, priority)));

        // k 個とそれ以降に分割し、間に新しいノードを挟んでマージ
        let root = self.root.take();
        let (left, right) = Self::split_at(root, k);
        let left_and_mid = Self::merge(left, new_node);
        self.root = Self::merge(left_and_mid, right);
    }

    /// 先頭から k 番目の要素を削除して返す (0-indexed)
    pub fn remove_at(&mut self, k: usize) -> Option<T> {
        if k >= self.len() {
            return None;
        }

        let root = self.root.take();
        // 1. [0 .. k-1] と [k .. N] に分割
        let (left, right) = Self::split_at(root, k);
        // 2. 右側をさらに [k] と [k+1 .. N] に分割 (つまり先頭1つを切り出す)
        let (target, right) = Self::split_at(right, 1);

        // 対象を取り除いて、残りをマージ
        self.root = Self::merge(left, right);

        // 取り出したノードの値を返す
        target.map(|node| node.value)
    }

    /// k番目の要素の参照を取得する
    pub fn get(&self, mut k: usize) -> Option<&T> {
        if k >= self.len() {
            return None;
        }
        let mut current = &self.root;
        while let Some(node) = current {
            let l_size = node.left.as_ref().map_or(0, |x| x.size);
            if k == l_size {
                return Some(&node.value);
            } else if k < l_size {
                current = &node.left;
            } else {
                k = k - l_size - 1;
                current = &node.right;
            }
        }
        None
    }
}

use std::fmt;

impl<T: fmt::Debug> fmt::Debug for ImplicitTreap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // デバッグ出力用のリストフォーマッタを作成
        let mut list = f.debug_list();

        // 通りがけ順 (in-order) でノードを巡回する再帰関数
        fn traverse<T: fmt::Debug>(node: &Option<Box<Node<T>>>, list: &mut fmt::DebugList<'_, '_>) {
            if let Some(n) = node {
                // 1. 左の子
                traverse(&n.left, list);
                // 2. 自分自身
                list.entry(&n.value);
                // 3. 右の子
                traverse(&n.right, list);
            }
        }

        // ルートから巡回を開始
        traverse(&self.root, &mut list);

        // リストを閉じて出力
        list.finish()
    }
}
