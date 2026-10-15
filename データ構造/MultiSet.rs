//! BTreeSet と同じ感覚で使える多重集合 (multiset)。
//!
//! 内部は `BTreeMap<T, usize>`（値 → 個数）で、総要素数を別に保持している。
//! 計算量は特記がなければ BTreeSet と同じ O(log n)。
//!
//! ```ignore
//! let mut ms = MultiSet::new();
//! ms.insert(3); ms.insert(1); ms.insert(3);
//! assert_eq!(ms.len(), 3);
//! assert_eq!(ms.count(&3), 2);
//! assert!(ms.remove(&3));            // 1個だけ消す
//! assert_eq!(ms.first(), Some(&1));
//! assert_eq!(ms.range(2..).next(), Some(&3));
//! ```

pub mod multiset {
    use std::borrow::Borrow;
    use std::collections::btree_map::{self, BTreeMap, Entry};
    use std::fmt;
    use std::iter::FusedIterator;
    use std::ops::{Bound, RangeBounds};

    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MultiSet<T> {
        map: BTreeMap<T, usize>,
        len: usize,
    }

    impl<T> Default for MultiSet<T> {
        fn default() -> Self {
            MultiSet {
                map: BTreeMap::new(),
                len: 0,
            }
        }
    }

    impl<T> MultiSet<T> {
        pub fn new() -> Self {
            Self::default()
        }

        /// 重複込みの要素数。O(1)
        pub fn len(&self) -> usize {
            self.len
        }

        /// 相異なる値の個数。O(1)
        pub fn unique_len(&self) -> usize {
            self.map.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn clear(&mut self) {
            self.map.clear();
            self.len = 0;
        }

        /// 昇順・重複込みのイテレータ（両端から使える）
        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                inner: MultiIter::new(self.map.iter()),
                remaining: self.len,
            }
        }

        /// (値, 個数) を昇順に返すイテレータ
        pub fn counts(&self) -> btree_map::Iter<'_, T, usize> {
            self.map.iter()
        }

        /// 相異なる値を昇順に返すイテレータ
        pub fn iter_unique(&self) -> btree_map::Keys<'_, T, usize> {
            self.map.keys()
        }

        /// 中身の BTreeMap<値, 個数> を参照する
        pub fn as_map(&self) -> &BTreeMap<T, usize> {
            &self.map
        }
    }

    impl<T: Ord> MultiSet<T> {
        /// 1個追加する。
        pub fn insert(&mut self, value: T) {
            self.insert_n(value, 1);
        }

        /// n 個まとめて追加する。n == 0 なら何もしない。
        pub fn insert_n(&mut self, value: T, n: usize) {
            if n == 0 {
                return;
            }
            *self.map.entry(value).or_insert(0) += n;
            self.len += n;
        }

        /// 1個だけ削除する。存在すれば true。
        pub fn remove<Q>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.remove_n(value, 1) == 1
        }

        /// 最大 n 個削除し、実際に削除した個数を返す。
        pub fn remove_n<Q>(&mut self, value: &Q, n: usize) -> usize
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            let removed = match self.map.get_mut(value) {
                None => return 0,
                Some(c) if *c > n => {
                    *c -= n;
                    n
                }
                Some(_) => self.map.remove(value).unwrap(),
            };
            self.len -= removed;
            removed
        }

        /// その値をすべて削除し、削除した個数を返す。
        pub fn remove_all<Q>(&mut self, value: &Q) -> usize
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            let removed = self.map.remove(value).unwrap_or(0);
            self.len -= removed;
            removed
        }

        /// 1個取り出して所有権ごと返す（最後の1個なら格納されていた値そのもの）。
        pub fn take<Q>(&mut self, value: &Q) -> Option<T>
        where
            T: Borrow<Q> + Clone,
            Q: Ord + ?Sized,
        {
            let c = self.map.get_mut(value)?;
            self.len -= 1;
            if *c > 1 {
                *c -= 1;
                // 値そのものを返すため map の中のキーを複製する
                self.map.get_key_value(value).map(|(k, _)| k.clone())
            } else {
                self.map.remove_entry(value).map(|(k, _)| k)
            }
        }

        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.map.contains_key(value)
        }

        /// その値の個数（無ければ 0）
        pub fn count<Q>(&self, value: &Q) -> usize
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.map.get(value).copied().unwrap_or(0)
        }

        /// 格納されている値への参照（BTreeSet::get 相当）
        pub fn get<Q>(&self, value: &Q) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.map.get_key_value(value).map(|(k, _)| k)
        }

        pub fn first(&self) -> Option<&T> {
            self.map.keys().next()
        }

        pub fn last(&self) -> Option<&T> {
            self.map.keys().next_back()
        }

        /// 最小値を1個取り出す。
        pub fn pop_first(&mut self) -> Option<T>
        where
            T: Clone,
        {
            let mut e = self.map.first_entry()?;
            self.len -= 1;
            if *e.get() > 1 {
                *e.get_mut() -= 1;
                Some(e.key().clone())
            } else {
                Some(e.remove_entry().0)
            }
        }

        /// 最大値を1個取り出す。
        pub fn pop_last(&mut self) -> Option<T>
        where
            T: Clone,
        {
            let mut e = self.map.last_entry()?;
            self.len -= 1;
            if *e.get() > 1 {
                *e.get_mut() -= 1;
                Some(e.key().clone())
            } else {
                Some(e.remove_entry().0)
            }
        }

        /// 範囲内を昇順・重複込みで返す（`.rev()` / `.next_back()` 可）
        pub fn range<Q, R>(&self, range: R) -> Range<'_, T>
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
            R: RangeBounds<Q>,
        {
            Range {
                inner: MultiIter::new(self.map.range(range)),
            }
        }

        /// 範囲内を (値, 個数) で返す
        pub fn range_counts<Q, R>(&self, range: R) -> btree_map::Range<'_, T, usize>
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
            R: RangeBounds<Q>,
        {
            self.map.range(range)
        }

        /// x 以上の最小値
        pub fn ge<Q>(&self, x: &Q) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.map
                .range((Bound::Included(x), Bound::Unbounded))
                .next()
                .map(|(k, _)| k)
        }

        /// x より大きい最小値
        pub fn gt<Q>(&self, x: &Q) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.map
                .range((Bound::Excluded(x), Bound::Unbounded))
                .next()
                .map(|(k, _)| k)
        }

        /// x 以下の最大値
        pub fn le<Q>(&self, x: &Q) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.map
                .range((Bound::Unbounded, Bound::Included(x)))
                .next_back()
                .map(|(k, _)| k)
        }

        /// x 未満の最大値
        pub fn lt<Q>(&self, x: &Q) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.map
                .range((Bound::Unbounded, Bound::Excluded(x)))
                .next_back()
                .map(|(k, _)| k)
        }

        /// f が false を返した値を（全個数ぶん）削除する。
        pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
            let mut len = self.len;
            self.map.retain(|k, c| {
                let keep = f(k);
                if !keep {
                    len -= *c;
                }
                keep
            });
            self.len = len;
        }

        /// other の中身をすべて移す（other は空になる）。
        /// 小さい方を大きい方へ移すので、マージテク（data structure on tree）にもそのまま使える。
        pub fn append(&mut self, other: &mut Self) {
            if self.unique_len() < other.unique_len() {
                std::mem::swap(self, other);
            }
            for (k, c) in std::mem::take(&mut other.map) {
                *self.map.entry(k).or_insert(0) += c;
            }
            self.len += other.len;
            other.len = 0;
        }

        /// key 以上の要素をすべて切り出して返す。
        pub fn split_off<Q>(&mut self, key: &Q) -> Self
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            let map = self.map.split_off(key);
            let moved: usize = map.values().sum();
            self.len -= moved;
            MultiSet { map, len: moved }
        }

        /// 値 v の個数を直接操作したいとき用（0 にしたら自動で消える）
        pub fn set_count(&mut self, value: T, n: usize) {
            match self.map.entry(value) {
                Entry::Occupied(mut e) => {
                    self.len -= *e.get();
                    if n == 0 {
                        e.remove();
                    } else {
                        *e.get_mut() = n;
                    }
                }
                Entry::Vacant(e) => {
                    if n > 0 {
                        e.insert(n);
                    }
                }
            }
            self.len += n;
        }
    }

    // ---------------------------------------------------------------
    // 借用イテレータ（重複込み、両端から使える）
    // ---------------------------------------------------------------

    struct MultiIter<'a, T, I> {
        inner: I,
        front: Option<(&'a T, usize)>,
        back: Option<(&'a T, usize)>,
    }

    impl<'a, T, I> MultiIter<'a, T, I> {
        fn new(inner: I) -> Self {
            MultiIter {
                inner,
                front: None,
                back: None,
            }
        }
    }

    impl<'a, T, I: Clone> Clone for MultiIter<'a, T, I> {
        fn clone(&self) -> Self {
            MultiIter {
                inner: self.inner.clone(),
                front: self.front,
                back: self.back,
            }
        }
    }

    fn take_one<'a, T>(slot: &mut Option<(&'a T, usize)>) -> Option<&'a T> {
        match slot {
            Some((v, c)) if *c > 0 => {
                *c -= 1;
                Some(*v)
            }
            _ => None,
        }
    }

    impl<'a, T: 'a, I> Iterator for MultiIter<'a, T, I>
    where
        I: Iterator<Item = (&'a T, &'a usize)>,
    {
        type Item = &'a T;
        fn next(&mut self) -> Option<&'a T> {
            loop {
                if let Some(v) = take_one(&mut self.front) {
                    return Some(v);
                }
                match self.inner.next() {
                    Some((v, &c)) => self.front = Some((v, c)),
                    None => return take_one(&mut self.back),
                }
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let f = self.front.map_or(0, |x| x.1);
            let b = self.back.map_or(0, |x| x.1);
            let (lo, _) = self.inner.size_hint();
            (f + b + lo, None)
        }
    }

    impl<'a, T: 'a, I> DoubleEndedIterator for MultiIter<'a, T, I>
    where
        I: DoubleEndedIterator<Item = (&'a T, &'a usize)>,
    {
        fn next_back(&mut self) -> Option<&'a T> {
            loop {
                if let Some(v) = take_one(&mut self.back) {
                    return Some(v);
                }
                match self.inner.next_back() {
                    Some((v, &c)) => self.back = Some((v, c)),
                    None => return take_one(&mut self.front),
                }
            }
        }
    }

    /// `MultiSet::iter` の戻り値
    pub struct Iter<'a, T> {
        inner: MultiIter<'a, T, btree_map::Iter<'a, T, usize>>,
        remaining: usize,
    }

    impl<'a, T> Clone for Iter<'a, T> {
        fn clone(&self) -> Self {
            Iter {
                inner: self.inner.clone(),
                remaining: self.remaining,
            }
        }
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<&'a T> {
            let r = self.inner.next();
            if r.is_some() {
                self.remaining -= 1;
            }
            r
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.remaining, Some(self.remaining))
        }
    }
    impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
        fn next_back(&mut self) -> Option<&'a T> {
            let r = self.inner.next_back();
            if r.is_some() {
                self.remaining -= 1;
            }
            r
        }
    }
    impl<'a, T> ExactSizeIterator for Iter<'a, T> {}
    impl<'a, T> FusedIterator for Iter<'a, T> {}

    /// `MultiSet::range` の戻り値
    pub struct Range<'a, T> {
        inner: MultiIter<'a, T, btree_map::Range<'a, T, usize>>,
    }

    impl<'a, T> Clone for Range<'a, T> {
        fn clone(&self) -> Self {
            Range {
                inner: self.inner.clone(),
            }
        }
    }

    impl<'a, T> Iterator for Range<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<&'a T> {
            self.inner.next()
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.inner.size_hint()
        }
    }
    impl<'a, T> DoubleEndedIterator for Range<'a, T> {
        fn next_back(&mut self) -> Option<&'a T> {
            self.inner.next_back()
        }
    }
    impl<'a, T> FusedIterator for Range<'a, T> {}

    impl<'a, T> IntoIterator for &'a MultiSet<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Iter<'a, T> {
            self.iter()
        }
    }

    // ---------------------------------------------------------------
    // 所有イテレータ（重複分は clone する）
    // ---------------------------------------------------------------

    pub struct IntoIter<T> {
        inner: btree_map::IntoIter<T, usize>,
        front: Option<(T, usize)>,
        back: Option<(T, usize)>,
        remaining: usize,
    }

    fn take_owned<T: Clone>(slot: &mut Option<(T, usize)>) -> Option<T> {
        match slot {
            Some((_, c)) if *c > 1 => {
                *c -= 1;
                slot.as_ref().map(|(v, _)| v.clone())
            }
            Some((_, 1)) => slot.take().map(|(v, _)| v),
            _ => None,
        }
    }

    impl<T: Clone> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<T> {
            loop {
                if let Some(v) = take_owned(&mut self.front) {
                    self.remaining -= 1;
                    return Some(v);
                }
                match self.inner.next() {
                    Some(kv) => self.front = Some(kv),
                    None => {
                        let r = take_owned(&mut self.back);
                        if r.is_some() {
                            self.remaining -= 1;
                        }
                        return r;
                    }
                }
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.remaining, Some(self.remaining))
        }
    }
    impl<T: Clone> DoubleEndedIterator for IntoIter<T> {
        fn next_back(&mut self) -> Option<T> {
            loop {
                if let Some(v) = take_owned(&mut self.back) {
                    self.remaining -= 1;
                    return Some(v);
                }
                match self.inner.next_back() {
                    Some(kv) => self.back = Some(kv),
                    None => {
                        let r = take_owned(&mut self.front);
                        if r.is_some() {
                            self.remaining -= 1;
                        }
                        return r;
                    }
                }
            }
        }
    }
    impl<T: Clone> ExactSizeIterator for IntoIter<T> {}
    impl<T: Clone> FusedIterator for IntoIter<T> {}

    impl<T: Clone> IntoIterator for MultiSet<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> IntoIter<T> {
            IntoIter {
                inner: self.map.into_iter(),
                front: None,
                back: None,
                remaining: self.len,
            }
        }
    }

    // ---------------------------------------------------------------
    // 変換・その他トレイト
    // ---------------------------------------------------------------

    impl<T: Ord> FromIterator<T> for MultiSet<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut ms = MultiSet::new();
            ms.extend(iter);
            ms
        }
    }

    impl<T: Ord> Extend<T> for MultiSet<T> {
        fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
            for v in iter {
                self.insert(v);
            }
        }
    }

    impl<'a, T: Ord + Copy + 'a> Extend<&'a T> for MultiSet<T> {
        fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
            for &v in iter {
                self.insert(v);
            }
        }
    }

    impl<T: Ord, const N: usize> From<[T; N]> for MultiSet<T> {
        fn from(arr: [T; N]) -> Self {
            arr.into_iter().collect()
        }
    }

    impl<T: fmt::Debug> fmt::Debug for MultiSet<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_set().entries(self.iter()).finish()
        }
    }
}
#[allow(unused_imports)]
pub use multiset::MultiSet;

// ===================================================================
// テスト（ライブラリに貼るときは以下は不要）
// ===================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn basic() {
        let mut ms = MultiSet::new();
        assert!(ms.is_empty());
        ms.insert(3);
        ms.insert(1);
        ms.insert(3);
        ms.insert_n(5, 3);
        assert_eq!(ms.len(), 6);
        assert_eq!(ms.unique_len(), 3);
        assert_eq!(ms.count(&3), 2);
        assert_eq!(ms.count(&4), 0);
        assert!(ms.contains(&5));
        assert_eq!(format!("{:?}", ms), "{1, 3, 3, 5, 5, 5}");

        assert!(ms.remove(&3));
        assert_eq!(ms.count(&3), 1);
        assert!(!ms.remove(&100));
        assert_eq!(ms.remove_n(&5, 2), 2);
        assert_eq!(ms.remove_n(&5, 10), 1);
        assert!(!ms.contains(&5));
        assert_eq!(ms.len(), 2);
        assert_eq!(ms.remove_all(&1), 1);
        assert_eq!(ms.len(), 1);
    }

    #[test]
    fn first_last_pop() {
        let mut ms: MultiSet<i32> = [4, 1, 1, 9, 9].into();
        assert_eq!(ms.first(), Some(&1));
        assert_eq!(ms.last(), Some(&9));
        assert_eq!(ms.pop_first(), Some(1));
        assert_eq!(ms.pop_first(), Some(1));
        assert_eq!(ms.pop_first(), Some(4));
        assert_eq!(ms.pop_last(), Some(9));
        assert_eq!(ms.len(), 1);
        assert_eq!(ms.pop_last(), Some(9));
        assert_eq!(ms.pop_last(), None);
        assert!(ms.is_empty());
    }

    #[test]
    fn iter_both_ends() {
        let ms: MultiSet<i32> = vec![2, 1, 2, 3, 3, 3].into_iter().collect();
        let v: Vec<_> = ms.iter().copied().collect();
        assert_eq!(v, vec![1, 2, 2, 3, 3, 3]);
        let r: Vec<_> = ms.iter().rev().copied().collect();
        assert_eq!(r, vec![3, 3, 3, 2, 2, 1]);
        assert_eq!(ms.iter().len(), 6);

        // 前後から交互に取る（同じキーを両側から食い合うケース）
        let one: MultiSet<i32> = [7, 7, 7].into();
        let mut it = one.iter();
        assert_eq!(it.next(), Some(&7));
        assert_eq!(it.next_back(), Some(&7));
        assert_eq!(it.next(), Some(&7));
        assert_eq!(it.next_back(), None);
        assert_eq!(it.next(), None);

        let mut it = ms.iter();
        let mut got = vec![];
        loop {
            match it.next() {
                Some(&x) => got.push(x),
                None => break,
            }
            match it.next_back() {
                Some(&x) => got.push(x),
                None => break,
            }
        }
        got.sort();
        assert_eq!(got, v);

        let owned: Vec<_> = ms.clone().into_iter().collect();
        assert_eq!(owned, v);
        let owned_rev: Vec<_> = ms.clone().into_iter().rev().collect();
        assert_eq!(owned_rev, r);
        let mut it = one.clone().into_iter();
        assert_eq!(it.next_back(), Some(7));
        assert_eq!(it.next(), Some(7));
        assert_eq!(it.next_back(), Some(7));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn range_and_bounds() {
        let ms: MultiSet<i64> = [1, 3, 3, 5, 7, 7].into();
        assert_eq!(ms.range(3..7).copied().collect::<Vec<_>>(), vec![3, 3, 5]);
        assert_eq!(
            ms.range(..=3).rev().copied().collect::<Vec<_>>(),
            vec![3, 3, 1]
        );
        assert_eq!(ms.range(4..).next(), Some(&5));
        assert_eq!(ms.range(..4).next_back(), Some(&3));
        assert_eq!(ms.ge(&3), Some(&3));
        assert_eq!(ms.gt(&3), Some(&5));
        assert_eq!(ms.le(&6), Some(&5));
        assert_eq!(ms.lt(&1), None);
        assert_eq!(ms.gt(&7), None);
        assert_eq!(ms.range_counts(3..=7).map(|(_, &c)| c).sum::<usize>(), 5);
    }

    #[test]
    fn strings_borrow() {
        let mut ms: MultiSet<String> = MultiSet::new();
        ms.insert("b".to_string());
        ms.insert("a".to_string());
        ms.insert("b".to_string());
        assert_eq!(ms.count("b"), 2);
        assert!(ms.contains("a"));
        assert_eq!(ms.take("b"), Some("b".to_string()));
        assert_eq!(ms.take("b"), Some("b".to_string()));
        assert_eq!(ms.take("b"), None);
        assert_eq!(ms.len(), 1);
    }

    #[test]
    fn retain_append_split() {
        let mut a: MultiSet<i32> = [1, 2, 2, 3, 4, 4].into();
        a.retain(|&x| x % 2 == 0);
        assert_eq!(a.iter().copied().collect::<Vec<_>>(), vec![2, 2, 4, 4]);
        assert_eq!(a.len(), 4);

        let mut b: MultiSet<i32> = [2, 5].into();
        b.append(&mut a); // 小さい b に大きい a を入れても OK
        assert!(a.is_empty() && a.len() == 0);
        assert_eq!(
            b.iter().copied().collect::<Vec<_>>(),
            vec![2, 2, 2, 4, 4, 5]
        );
        assert_eq!(b.len(), 6);

        let hi = b.split_off(&4);
        assert_eq!(b.len(), 3);
        assert_eq!(hi.len(), 3);
        assert_eq!(hi.iter().copied().collect::<Vec<_>>(), vec![4, 4, 5]);

        b.set_count(2, 0);
        assert!(b.is_empty());
        b.set_count(9, 4);
        assert_eq!(b.len(), 4);
        b.set_count(9, 1);
        assert_eq!(b.len(), 1);
    }

    /// ランダム操作で「ソート済み Vec」と突き合わせる
    #[test]
    fn random_vs_vec() {
        let mut seed: u64 = 88172645463325252;
        let mut rnd = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..50 {
            let mut ms = MultiSet::new();
            let mut v: Vec<u64> = vec![];
            for _ in 0..2000 {
                let x = rnd() % 30;
                match rnd() % 6 {
                    0 | 1 => {
                        ms.insert(x);
                        let p = v.partition_point(|&y| y < x);
                        v.insert(p, x);
                    }
                    2 => {
                        let r = ms.remove(&x);
                        if let Some(p) = v.iter().position(|&y| y == x) {
                            v.remove(p);
                            assert!(r);
                        } else {
                            assert!(!r);
                        }
                    }
                    3 => {
                        assert_eq!(
                            ms.pop_first(),
                            if v.is_empty() {
                                None
                            } else {
                                Some(v.remove(0))
                            }
                        );
                    }
                    4 => {
                        assert_eq!(ms.pop_last(), v.pop());
                    }
                    _ => {
                        let lo = x;
                        let hi = rnd() % 30;
                        let exp: Vec<u64> =
                            v.iter().copied().filter(|&y| lo <= y && y < hi).collect();
                        if lo <= hi {
                            assert_eq!(ms.range(lo..hi).copied().collect::<Vec<_>>(), exp);
                        }
                        assert_eq!(ms.ge(&x).copied(), v.iter().copied().find(|&y| y >= x));
                        assert_eq!(ms.lt(&x).copied(), v.iter().rev().copied().find(|&y| y < x));
                    }
                }
                assert_eq!(ms.len(), v.len());
                assert_eq!(ms.count(&x), v.iter().filter(|&&y| y == x).count());
            }
            assert_eq!(ms.iter().copied().collect::<Vec<_>>(), v);
            let uniq: BTreeSet<u64> = v.iter().copied().collect();
            assert_eq!(
                ms.iter_unique().copied().collect::<Vec<_>>(),
                uniq.into_iter().collect::<Vec<_>>()
            );
        }
    }
}
