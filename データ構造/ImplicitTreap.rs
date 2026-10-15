use ac_library::MapMonoid;

struct SetMapMonoid;

impl MapMonoid for SetMapMonoid {
    type M = Additive<i64>; // 上で定義した Monoid
    type F = i64; // 更新操作の型（ここでは "値をセットする"）

    fn identity_map() -> Self::F {
        i64::MIN // 無効な値を表す
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if *f == i64::MIN {
            *x // 更新なし
        } else {
            *f // f を代入
        }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        if *f == i64::MIN {
            *g
        } else {
            *f
        }
    }
}

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

struct Node<M: MapMonoid> {
    val: <M::M as Monoid>::S,
    acc: <M::M as Monoid>::S,
    lazy: M::F,
    has_lazy: bool, // FにPartialEqがないため、遅延作用素の有無をフラグで管理する
    rev: bool,
    priority: u64,
    size: usize,
    left: Option<Box<Node<M>>>,
    right: Option<Box<Node<M>>>,
}

impl<M: MapMonoid> Node<M> {
    fn new(val: <M::M as Monoid>::S, priority: u64) -> Self {
        Node {
            acc: val.clone(),
            val,
            lazy: M::identity_map(),
            has_lazy: false,
            rev: false,
            priority,
            size: 1,
            left: None,
            right: None,
        }
    }

    /// 【Push】: 遅延評価フラグと作用素を子へ伝播
    fn push(&mut self) {
        // 1. 区間反転の伝播
        if self.rev {
            let l = self.left.take();
            let r = self.right.take();

            // 左右の子を入れ替えつつ、子へフラグを伝播
            if let Some(mut node) = l {
                node.rev ^= true;
                self.right = Some(node);
            } else {
                self.right = None;
            }

            if let Some(mut node) = r {
                node.rev ^= true;
                self.left = Some(node);
            } else {
                self.left = None;
            }

            self.rev = false;
        }

        // 2. 作用素の伝播
        if self.has_lazy {
            if let Some(ref mut l) = self.left {
                l.val = M::mapping(&self.lazy, &l.val);
                l.acc = M::mapping(&self.lazy, &l.acc);
                if l.has_lazy {
                    l.lazy = M::composition(&self.lazy, &l.lazy);
                } else {
                    l.lazy = self.lazy.clone();
                    l.has_lazy = true;
                }
            }
            if let Some(ref mut r) = self.right {
                r.val = M::mapping(&self.lazy, &r.val);
                r.acc = M::mapping(&self.lazy, &r.acc);
                if r.has_lazy {
                    r.lazy = M::composition(&self.lazy, &r.lazy);
                } else {
                    r.lazy = self.lazy.clone();
                    r.has_lazy = true;
                }
            }
            self.lazy = M::identity_map();
            self.has_lazy = false;
        }
    }

    /// 【Update】: 子の状態から自身の集約値を再計算
    fn update(&mut self) {
        self.size = 1;

        let l_acc = self
            .left
            .as_ref()
            .map(|n| n.acc.clone())
            .unwrap_or_else(M::identity_element);
        let r_acc = self
            .right
            .as_ref()
            .map(|n| n.acc.clone())
            .unwrap_or_else(M::identity_element);

        // 左の子 + 自分 + 右の子 の順番でマージ
        let l_mid = M::binary_operation(&l_acc, &self.val);
        self.acc = M::binary_operation(&l_mid, &r_acc);

        self.size += self.left.as_ref().map_or(0, |n| n.size);
        self.size += self.right.as_ref().map_or(0, |n| n.size);
    }
}

pub struct ImplicitTreap<M: MapMonoid> {
    root: Option<Box<Node<M>>>,
    rng: XorShift,
}

impl<M: MapMonoid> ImplicitTreap<M> {
    pub fn new() -> Self {
        ImplicitTreap {
            root: None,
            rng: XorShift::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |n| n.size)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn split_at(
        node: Option<Box<Node<M>>>,
        k: usize,
    ) -> (Option<Box<Node<M>>>, Option<Box<Node<M>>>) {
        match node {
            None => (None, None),
            Some(mut n) => {
                n.push();
                let l_size = n.left.as_ref().map_or(0, |x| x.size);
                if k <= l_size {
                    let (l, r) = Self::split_at(n.left.take(), k);
                    n.left = r;
                    n.update();
                    (l, Some(n))
                } else {
                    let (l, r) = Self::split_at(n.right.take(), k - l_size - 1);
                    n.right = l;
                    n.update();
                    (Some(n), r)
                }
            }
        }
    }

    fn merge(left: Option<Box<Node<M>>>, right: Option<Box<Node<M>>>) -> Option<Box<Node<M>>> {
        match (left, right) {
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                l.push();
                r.push();
                if l.priority > r.priority {
                    l.right = Self::merge(l.right.take(), Some(r));
                    l.update();
                    Some(l)
                } else {
                    r.left = Self::merge(Some(l), r.left.take());
                    r.update();
                    Some(r)
                }
            }
        }
    }

    pub fn insert_at(&mut self, k: usize, val: <M::M as Monoid>::S) {
        let new_node = Some(Box::new(Node::new(val, self.rng.next())));
        let (l, r) = Self::split_at(self.root.take(), k);
        self.root = Self::merge(Self::merge(l, new_node), r);
    }

    pub fn remove_at(&mut self, k: usize) {
        if k >= self.len() {
            return;
        }
        let (l, right) = Self::split_at(self.root.take(), k);
        let (_, r) = Self::split_at(right, 1);
        self.root = Self::merge(l, r);
    }

    /// 区間 [l, r) に作用素 f を適用する (遅延評価)
    pub fn apply(&mut self, l: usize, r: usize, f: M::F) {
        if l >= r {
            return;
        }
        let (left_mid, right) = Self::split_at(self.root.take(), r);
        let (left, mut mid) = Self::split_at(left_mid, l);

        if let Some(ref mut m) = mid {
            m.val = M::mapping(&f, &m.val);
            m.acc = M::mapping(&f, &m.acc);
            if m.has_lazy {
                m.lazy = M::composition(&f, &m.lazy);
            } else {
                m.lazy = f.clone();
                m.has_lazy = true;
            }
        }

        self.root = Self::merge(Self::merge(left, mid), right);
    }

    /// 区間 [l, r) の集約値を取得する
    pub fn query(&mut self, l: usize, r: usize) -> <M::M as Monoid>::S {
        if l >= r {
            return M::identity_element();
        }
        let (left_mid, right) = Self::split_at(self.root.take(), r);
        let (left, mid) = Self::split_at(left_mid, l);

        let ans = mid
            .as_ref()
            .map_or(M::identity_element(), |m| m.acc.clone());

        self.root = Self::merge(Self::merge(left, mid), right);
        ans
    }

    /// 区間 [l, r) を反転させる (可換な演算でのみ安全に利用可能)
    pub fn reverse(&mut self, l: usize, r: usize) {
        if l >= r {
            return;
        }
        let (left_mid, right) = Self::split_at(self.root.take(), r);
        let (left, mut mid) = Self::split_at(left_mid, l);

        if let Some(ref mut m) = mid {
            m.rev ^= true;
            // 警告: 非可換モノイドの場合、ここで m.acc の左右を反転させる必要がありますが、
            // Trait に対応する関数がないためそのままにしています。
        }

        self.root = Self::merge(Self::merge(left, mid), right);
    }
}

use std::fmt;

impl<M: MapMonoid> fmt::Debug for ImplicitTreap<M>
where
    // 要素 S が Debug を実装している場合のみ {:?} を許可
    <M::M as Monoid>::S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut is_first = true;
        // 木を書き換えずに、遅延タグを計算しながら読み取る
        Self::fmt_vec_rec(
            &self.root,
            f,
            M::identity_map(),
            false,
            false,
            &mut is_first,
        )?;
        write!(f, "]")
    }
}

impl<M: MapMonoid> ImplicitTreap<M> {
    /// 読み取り専用で論理的な配列状態をフォーマットするヘルパー関数
    fn fmt_vec_rec(
        node: &Option<Box<Node<M>>>,
        f: &mut fmt::Formatter<'_>,
        parent_lazy: M::F,
        parent_has_lazy: bool,
        parent_rev: bool,
        is_first: &mut bool,
    ) -> fmt::Result
    where
        <M::M as Monoid>::S: fmt::Debug,
    {
        if let Some(n) = node {
            // 1. 親から降りてきた反転フラグと遅延タグを、現在のノードのものと合成する
            let current_rev = parent_rev ^ n.rev;

            let (current_lazy, current_has_lazy) = if parent_has_lazy {
                if n.has_lazy {
                    (M::composition(&parent_lazy, &n.lazy), true)
                } else {
                    (parent_lazy.clone(), true)
                }
            } else {
                (n.lazy.clone(), n.has_lazy)
            };

            // 2. 左の子へ進む (反転フラグが立っている場合は右の子へ進む)
            let left_child = if current_rev { &n.right } else { &n.left };
            Self::fmt_vec_rec(
                left_child,
                f,
                current_lazy.clone(),
                current_has_lazy,
                current_rev,
                is_first,
            )?;

            // 3. 現在のノードの真の値を計算して出力
            if !*is_first {
                write!(f, ", ")?;
            }
            *is_first = false;

            let actual_val = if current_has_lazy {
                M::mapping(&current_lazy, &n.val)
            } else {
                n.val.clone()
            };
            write!(f, "{:?}", actual_val)?;

            // 4. 右の子へ進む (反転フラグが立っている場合は左の子へ進む)
            let right_child = if current_rev { &n.left } else { &n.right };
            Self::fmt_vec_rec(
                right_child,
                f,
                current_lazy.clone(),
                current_has_lazy,
                current_rev,
                is_first,
            )?;
        }
        Ok(())
    }
}
