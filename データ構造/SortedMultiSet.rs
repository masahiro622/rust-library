use std::ops::{Bound, RangeBounds};

/// 平方分割によるSortedMultiset
/// 挿入、削除、検索、k番目の要素の取得などが O(√N) で可能
#[derive(Debug, Clone)]
pub struct SortedMultiset<T> {
    a: Vec<Vec<T>>,
    size: usize,
}

impl<T: Ord + Clone> SortedMultiset<T> {
    const BUCKET_RATIO: usize = 16;
    const SPLIT_RATIO: usize = 24;

    /// Iterableから新しいSortedMultisetを作成する
    /// O(N log N)
    pub fn new(source: Vec<T>) -> Self {
        let n = source.len();
        let mut a_flat = source;
        a_flat.sort_unstable();

        let num_bucket = if n == 0 {
            1 // 空の場合はバケット1つにしておく（insertの実装簡易化のため）
        } else {
            let ratio = Self::BUCKET_RATIO as f64;
            (n as f64 / ratio).sqrt().ceil() as usize
        };

        let mut a = Vec::with_capacity(num_bucket);
        if n == 0 {
            a.push(vec![]);
        } else {
            // 均等に分割
            for chunk in a_flat.chunks(n.div_ceil(num_bucket)) {
                a.push(chunk.to_vec());
            }
        }

        Self { a, size: n }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// 最小の要素への参照を返す / O(1)
    pub fn first(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.a.first()?.first()
    }

    /// 最大の要素への参照を返す / O(1)
    pub fn last(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.a.last()?.last()
    }

    /// 全要素をベクタとして返す
    pub fn to_vec(&self) -> Vec<T> {
        let mut res = Vec::with_capacity(self.size);
        for bucket in &self.a {
            res.extend_from_slice(bucket);
        }
        res
    }

    /// 内部的にバケットの位置と、バケット内のインデックスを特定する
    /// 返り値: (バケットのインデックス, バケット内のインデックス)
    fn position(&self, x: &T) -> (usize, usize) {
        for (i, bucket) in self.a.iter().enumerate() {
            if bucket.is_empty() {
                continue;
            }
            if x <= &bucket[bucket.len() - 1] {
                // bisect_left
                let pos = bucket.partition_point(|v| v < x);
                return (i, pos);
            }
        }
        // 最後のバケットの末尾
        (self.a.len() - 1, self.a.last().map_or(0, |b| b.len()))
    }

    pub fn contains(&self, x: &T) -> bool {
        if self.size == 0 {
            return false;
        }
        let (b_idx, i) = self.position(x);
        let bucket = &self.a[b_idx];
        i < bucket.len() && &bucket[i] == x
    }

    /// xの個数をカウントする
    pub fn count(&self, x: &T) -> usize {
        self.rank(x..=x)
    }

    /// 要素を追加する / O(√N)
    pub fn insert(&mut self, x: T) {
        if self.size == 0 {
            self.a = vec![vec![x]];
            self.size = 1;
            return;
        }

        let (b_idx, i) = self.position(&x);
        self.a[b_idx].insert(i, x);
        self.size += 1;

        // バケットが大きくなりすぎたら分割
        if self.a[b_idx].len() > self.a.len() * Self::SPLIT_RATIO {
            let mid = self.a[b_idx].len() / 2;
            let right_part = self.a[b_idx].split_off(mid);
            self.a.insert(b_idx + 1, right_part);
        }
    }

    fn pop_inner(&mut self, b_idx: usize, i: usize) -> T {
        let ans = self.a[b_idx].remove(i);
        self.size -= 1;
        if self.a[b_idx].is_empty() && self.a.len() > 1 {
            self.a.remove(b_idx);
        }
        ans
    }

    /// 要素を削除し、成功したらtrueを返す / O(√N)
    pub fn remove(&mut self, x: &T) -> bool {
        if self.size == 0 {
            return false;
        }
        let (b_idx, i) = self.position(x);
        if i == self.a[b_idx].len() || &self.a[b_idx][i] != x {
            return false;
        }
        self.pop_inner(b_idx, i);
        true
    }

    /// x に一致する要素をすべて削除し、削除した件数を返す / O(√N + count(x))
    pub fn remove_all(&mut self, x: &T) -> usize {
        if self.size == 0 {
            return 0;
        }
        let mut removed = 0;
        let (mut b_idx, mut i) = self.position(x);
        loop {
            let bucket_len = self.a[b_idx].len();
            if i >= bucket_len || &self.a[b_idx][i] != x {
                break;
            }
            let end = self.a[b_idx][i..].partition_point(|v| v == x) + i;
            self.size -= end - i;
            removed += end - i;
            self.a[b_idx].drain(i..end);

            let reached_bucket_end = end == bucket_len;
            if self.a[b_idx].is_empty() && self.a.len() > 1 {
                self.a.remove(b_idx);
            } else {
                b_idx += 1;
            }
            i = 0;

            if !reached_bucket_end || b_idx >= self.a.len() {
                break;
            }
        }
        removed
    }

    /// 最小の要素を削除して返す / O(√N)
    pub fn pop_first(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        Some(self.pop_inner(0, 0))
    }

    /// 最大の要素を削除して返す / O(√N)
    pub fn pop_last(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let b_idx = self.a.len() - 1;
        let i = self.a[b_idx].len() - 1;
        Some(self.pop_inner(b_idx, i))
    }

    /// i番目の要素を取得する (0-indexed)
    pub fn nth(&self, mut i: usize) -> Option<&T> {
        if i >= self.size {
            return None;
        }
        for bucket in &self.a {
            if i < bucket.len() {
                return Some(&bucket[i]);
            }
            i -= bucket.len();
        }
        None
    }

    /// i番目の要素を削除して返す
    pub fn pop_index(&mut self, mut i: usize) -> Option<T> {
        if i >= self.size {
            return None;
        }
        for b_idx in 0..self.a.len() {
            if i < self.a[b_idx].len() {
                return Some(self.pop_inner(b_idx, i));
            }
            i -= self.a[b_idx].len();
        }
        None
    }

    /// pred(v) を満たす要素が先頭から連続する個数を返す
    /// (pred はソート順に対して単調: true が続いた後に false が続く必要がある)
    fn count_where(&self, pred: impl Fn(&T) -> bool) -> usize {
        let mut ans = 0;
        for bucket in &self.a {
            if bucket.is_empty() {
                continue;
            }
            if !pred(&bucket[bucket.len() - 1]) {
                return ans + bucket.partition_point(|v| pred(v));
            }
            ans += bucket.len();
        }
        ans
    }

    /// range に含まれる要素の個数を返す
    /// 例: rank(a..b), rank(a..=b), rank(..b), rank(a..), rank(..) など
    pub fn rank<R: RangeBounds<T>>(&self, range: R) -> usize {
        let left = match range.start_bound() {
            Bound::Included(x) => self.count_where(|v| v < x),
            Bound::Excluded(x) => self.count_where(|v| v <= x),
            Bound::Unbounded => 0,
        };
        let right = match range.end_bound() {
            Bound::Included(x) => self.count_where(|v| v <= x),
            Bound::Excluded(x) => self.count_where(|v| v < x),
            Bound::Unbounded => self.size,
        };
        right - left
    }

    /// range に含まれる要素を昇順で走査する DoubleEndedIterator を返す
    /// BTreeSet::range と同様に使う
    /// 例: range(a..b), range(a..=b), range(..b), range(a..), range(..) など
    ///
    pub fn range<R: RangeBounds<T>>(&self, range: R) -> Range<'_, T> {
        let front = match range.start_bound() {
            Bound::Included(x) => self.count_where(|v| v < x),
            Bound::Excluded(x) => self.count_where(|v| v <= x),
            Bound::Unbounded => 0,
        };
        let back = match range.end_bound() {
            Bound::Included(x) => self.count_where(|v| v <= x),
            Bound::Excluded(x) => self.count_where(|v| v < x),
            Bound::Unbounded => self.size,
        };
        Range {
            set: self,
            front,
            back,
        }
    }

    /// 全要素を昇順で走査する DoubleEndedIterator を返す（`range(..)` のエイリアス）
    pub fn iter(&self) -> Range<'_, T> {
        self.range(..)
    }
}

impl<T: Ord + Clone> Default for SortedMultiset<T> {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl<T: Ord + Clone> PartialEq for SortedMultiset<T> {
    /// 内部のバケット分割ではなく、要素列が一致するかどうかで比較する
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.a.iter().flatten().eq(other.a.iter().flatten())
    }
}

impl<T: Ord + Clone> Eq for SortedMultiset<T> {}

impl<T: Ord + Clone> FromIterator<T> for SortedMultiset<T> {
    /// Iteratorから新しいSortedMultisetを作成する / O(N log N)
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<T: Ord + Clone> Extend<T> for SortedMultiset<T> {
    /// 要素をまとめて追加する / 1要素あたり O(√N)
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.insert(x);
        }
    }
}

impl<'a, T: Ord + Clone> IntoIterator for &'a SortedMultiset<T> {
    type Item = &'a T;
    type IntoIter = Range<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.range(..)
    }
}

/// [`SortedMultiset::range`] が返すイテレータ / O(√N) per next()/next_back()
pub struct Range<'a, T> {
    set: &'a SortedMultiset<T>,
    front: usize,
    back: usize,
}

impl<'a, T: Ord + Clone> Iterator for Range<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        let item = self.set.nth(self.front);
        self.front += 1;
        item
    }
}

impl<'a, T: Ord + Clone> DoubleEndedIterator for Range<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        self.set.nth(self.back)
    }
}
