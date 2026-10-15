use num_traits::{Bounded, One};
use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::Add;

/// 区間ノード
#[derive(Clone, Debug)]
pub struct Node<S, T> {
    pub l: T,
    pub r: T,
    pub x: S,
}

impl<S, T> Node<S, T> {
    pub fn new(l: T, r: T, x: S) -> Self {
        Self { l, r, x }
    }
}

/// IntervalManager
///
/// S: 区間に格納する値（Clone + PartialEq）
/// T: 区間の型（Ord + Copy + Add<Output=T> + One + Bounded）
pub struct IntervalManager<S, T>
where
    S: Clone + PartialEq + Debug,
    T: Ord + Copy + Add<Output = T> + One + Bounded + Debug,
{
    unit: S,
    map: BTreeMap<T, Node<S, T>>,
}

impl<S, T> IntervalManager<S, T>
where
    S: Clone + PartialEq + Debug,
    T: Ord + Copy + Add<Output = T> + One + Bounded + Debug,
{
    /// [l, r) := x を行う（add/del コールバック付き, 1クロージャ版）
    pub fn update_with_combined<F>(&mut self, l: T, r: T, x: S, f: F)
    where
        F: FnMut(bool, T, T, S),
    {
        use std::cell::RefCell;
        let f = RefCell::new(f);
        self.update_with(
            l,
            r,
            x,
            |l, r, x| (f.borrow_mut())(true, l, r, x),
            |l, r, x| (f.borrow_mut())(false, l, r, x),
        );
    }
}
impl<S, T> IntervalManager<S, T>
where
    S: Clone + PartialEq + Debug,
    T: Ord + Copy + Add<Output = T> + One + Bounded + Debug,
{
    /// 単位元 unit を指定して空で作成
    pub fn new(unit: S) -> Self {
        Self {
            unit,
            map: BTreeMap::new(),
        }
    }

    /// ベクタ値からの初期化（C++ の vector<T> コンストラクタに対応）
    /// ここではインデックスが usize で T: From<usize> を要する簡易版
    pub fn from_vec_values(values: &[S]) -> Self
    where
        T: From<usize>,
    {
        let unit = values
            .get(0)
            .cloned()
            .unwrap_or_else(|| panic!("empty and no unit"));
        let mut map = BTreeMap::new();
        let mut i = 0usize;
        while i < values.len() {
            let mut j = i + 1;
            while j < values.len() && values[j] == values[i] {
                j += 1;
            }
            let l: T = T::from(i);
            let r: T = T::from(j);
            map.insert(l, Node::new(l, r, values[i].clone()));
            i = j;
        }
        Self { unit, map }
    }

    /// x を含む区間ノード（見つからなければ [x, x+1) を unit で返す）
    pub fn get_seg(&self, x: T) -> Node<S, T> {
        if let Some((_k, node)) = self
            .map
            .range(..=x)
            .next_back()
            .map(|(k, v)| (*k, v.clone()))
        {
            if node.l <= x && x < node.r {
                return node;
            }
        }
        Node::new(x, x + T::one(), self.unit.clone())
    }

    /// x に設定されている値（なければ unit）
    pub fn get(&self, x: T) -> S {
        if let Some((_k, node)) = self
            .map
            .range(..=x)
            .next_back()
            .map(|(k, v)| (*k, v.clone()))
        {
            if node.l <= x && x < node.r {
                return node.x;
            }
        }
        self.unit.clone()
    }

    /// [l, r) := x を行う（add/del コールバック付き）
    ///
    /// add: 追加される区間に対して呼ばれる (l, r, x)
    /// del: 消される区間に対して呼ばれる (l, r, old_x)
    pub fn update_with<F_add, F_del>(
        &mut self,
        mut l: T,
        mut r: T,
        x: S,
        mut add: F_add,
        mut del: F_del,
    ) where
        F_add: FnMut(T, T, S),
        F_del: FnMut(T, T, S),
    {
        // 1) l 以上の区間で r にかかるものを処理（C++ の while 部分）
        loop {
            let next_key_opt: Option<T> = {
                // borrow が残らないようにここで取得してコピーする
                self.map
                    .range((std::ops::Bound::Included(&l), std::ops::Bound::Unbounded))
                    .next()
                    .map(|(k, _v)| *k)
            };
            let key = match next_key_opt {
                None => break,
                Some(k) => k,
            };
            if key > r {
                break;
            }
            let node = {
                // clone node を先に取る
                self.map.get(&key).expect("key must exist").clone()
            };

            if node.l == r {
                if node.x == x {
                    del(r, node.r, x.clone());
                    r = node.r;
                    self.map.remove(&node.l);
                }
                break;
            }

            if node.r <= r {
                // node が完全に [l,r) に含まれる
                del(node.l, node.r, node.x.clone());
                self.map.remove(&node.l);
            } else {
                // node.r > r の場合
                if node.x == x {
                    // 右側と結合
                    r = node.r;
                    del(node.l, node.r, node.x.clone());
                    self.map.remove(&node.l);
                } else {
                    // 一部だけ被る: node -> [node.l, r) と [r, node.r)
                    del(node.l, r, node.x.clone());
                    self.map.remove(&node.l);
                    self.map.insert(r, Node::new(r, node.r, node.x.clone()));
                }
            }
        }

        // 2) 直前の区間（l の左側）をチェック
        if let Some((prev_k, node)) = self
            .map
            .range(..=l)
            .next_back()
            .map(|(k, v)| (*k, v.clone()))
        {
            let node = node; // clone
            if node.r == l {
                // 隣接
                if node.x == x {
                    del(node.l, node.r, node.x.clone());
                    l = node.l;
                    self.map.remove(&node.l);
                }
            } else if l < node.r {
                // 左側と重なっている
                if node.x == x {
                    del(node.l, node.r, node.x.clone());
                    l = min(l, node.l);
                    r = max(r, node.r);
                    self.map.remove(&node.l);
                } else {
                    // node.x != x
                    if r < node.r {
                        // 右側を分割して残す [r, node.r)
                        self.map.insert(r, Node::new(r, node.r, node.x.clone()));
                    }
                    // 削除コールバック: [l, min(r, node.r))
                    let cut_r = if r < node.r { r } else { node.r };
                    del(l, cut_r, node.x.clone());
                    // node を左側のみ [node.l, l) に置き換え
                    self.map.remove(&node.l);
                    self.map
                        .insert(node.l, Node::new(node.l, l, node.x.clone()));
                }
            }
        }

        // 最後に新しい区間を追加して add を呼ぶ
        add(l, r, x.clone());
        self.map.insert(l, Node::new(l, r, x));
    }

    /// add/del なし版
    pub fn update(&mut self, l: T, r: T, x: S) {
        self.update_with(l, r, x, |_l, _r, _x| {}, |_l, _r, _x| {});
    }

    /// デバッグ表示（S, T が Display を満たす場合）
    pub fn show(&self)
    where
        S: std::fmt::Display,
        T: std::fmt::Display,
    {
        for (_k, e) in &self.map {
            eprintln!("[{}, {}): {}", e.l, e.r, e.x);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_update_get() {
        let mut im = IntervalManager::new(0i32); // unit = 0
        im.update(0i32, 3i32, 1i32);
        im.update(3i32, 5i32, 2i32);

        assert_eq!(im.get(0), 1);
        assert_eq!(im.get(2), 1);
        assert_eq!(im.get(3), 2);
        assert_eq!(im.get(4), 2);
        assert_eq!(im.get(10), 0); // unit
    }

    #[test]
    fn callback_merge_split() {
        let mut im = IntervalManager::new(0i32);
        im.update(0, 3, 1);
        im.update(3, 6, 1);
        // ここで [0,6) が値 1 に結合される想定
        im.update_with(
            2,
            4,
            1,
            |l, r, x| {
                // add called for final insertion
                assert_eq!((l, r, x), (0, 6, 1));
            },
            |_l, _r, _x| {},
        );
    }
}
