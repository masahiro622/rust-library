use std::collections::BTreeMap;
use std::fmt::Debug;

/// 区間を表す構造体
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interval<T> {
    pub start: T,
    pub end: T, // 排他的終端 [start, end)
}

impl<T> Interval<T>
where
    T: Copy + PartialOrd,
{
    /// 新しい区間を作成
    #[inline]
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }

    /// 区間が空かどうか
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// 点が区間に含まれるかチェック
    #[inline]
    pub fn contains(&self, point: T) -> bool {
        self.start <= point && point < self.end
    }

    /// 他の区間と重複するかチェック
    #[inline]
    pub fn overlaps(&self, other: &Interval<T>) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// 他の区間と隣接するかチェック（結合可能）
    #[inline]
    pub fn is_adjacent(&self, other: &Interval<T>) -> bool {
        self.end == other.start || other.end == self.start
    }

    /// 他の区間と結合可能かチェック
    #[inline]
    pub fn can_merge(&self, other: &Interval<T>) -> bool {
        self.overlaps(other) || self.is_adjacent(other)
    }
}

impl<T> Interval<T>
where
    T: Copy + Ord,
{
    /// 他の区間とのマージ
    #[inline]
    pub fn merge(&self, other: &Interval<T>) -> Interval<T> {
        Interval {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

/// 高速化されたIntervalSet
#[derive(Debug, Clone)]
pub struct IntervalSet<T> {
    intervals: BTreeMap<T, T>, // start -> end のマッピング
}

impl<T> Default for IntervalSet<T>
where
    T: Copy + Ord + Debug + std::ops::Sub<Output = T>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntervalSet<T>
where
    T: Copy + Ord + Debug + std::ops::Sub<Output = T>,
{
    /// 新しい IntervalSet を作成
    #[inline]
    pub fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
        }
    }

    /// 区間を挿入し、今回の挿入で「新たに覆われた長さ」を返す
    /// 計算量: O(log n + k)
    pub fn insert(&mut self, new_interval: Interval<T>) -> T {
        if new_interval.is_empty() {
            return new_interval.end - new_interval.start; // = 0
        }

        let orig_start = new_interval.start;
        let orig_end = new_interval.end;

        // 1) [orig_start, orig_end) 内で既存に覆われているユニオン長を求める
        let mut uncovered = orig_end - orig_start;

        // 候補は「直前の1区間」と「開始点が [orig_start, orig_end) の区間」
        let mut segs: Vec<(T, T)> = Vec::new();

        if let Some((_, &e)) = self.intervals.range(..=orig_start).next_back() {
            if e > orig_start {
                // 交差部分をクリップ
                let cs = orig_start;
                let ce = if e < orig_end { e } else { orig_end };
                segs.push((cs, ce));
            }
        }
        for (&s, &e) in self.intervals.range(orig_start..orig_end) {
            let cs = s; // s >= orig_start
            let ce = if e < orig_end { e } else { orig_end };
            if cs < ce {
                segs.push((cs, ce));
            }
        }

        // クリップ済み区間をマージしてユニオン長を引く
        // segs は開始昇順になっている
        let mut cur: Option<(T, T)> = None;
        for (s, e) in segs {
            if let Some((cs, ce)) = cur {
                if s <= ce {
                    let ne = if e > ce { e } else { ce };
                    cur = Some((cs, ne));
                } else {
                    uncovered = uncovered - (ce - cs);
                    cur = Some((s, e));
                }
            } else {
                cur = Some((s, e));
            }
        }
        if let Some((cs, ce)) = cur {
            uncovered = uncovered - (ce - cs);
        }
        let added_len = uncovered;

        // 2) 実際の区間集合をマージ更新
        let mut start = orig_start;
        let mut end = orig_end;

        // 左側に重なり/隣接があれば取り込む（remove して進む）
        if let Some((&ps, &pe)) = self.intervals.range(..=start).next_back() {
            if pe >= start {
                start = ps;
                if pe > end {
                    end = pe;
                }
                self.intervals.remove(&ps);
            }
        }

        // 右側は s <= end の間取り込み続ける（毎回 remove して次へ）
        while let Some((&s, &e)) = self.intervals.range(start..=end).next() {
            // s は常に現在の end 以下
            if e > end {
                end = e;
            }
            self.intervals.remove(&s);
        }

        // マージ後の区間を挿入
        self.intervals.insert(start, end);

        added_len
    }

    /// 区間を削除し、今回の削除で「実際に取り除かれた長さ（ユニオン長）」を返す
    /// 計算量: O(log n + k) （k は今回重なった区間の数）
    pub fn remove(&mut self, target: Interval<T>) -> T {
        if target.is_empty() {
            return target.end - target.start; // = 0
        }

        // 1) [target.start, target.end) と既存の重複ユニオン長を計算
        let mut removed = target.end - target.start;

        // 候補は「直前の1区間」と「開始点が [start, end) の区間」
        let mut segs: Vec<(T, T)> = Vec::new();
        let s0 = target.start;
        let e0 = target.end;

        if let Some((_, &e)) = self.intervals.range(..=s0).next_back() {
            if e > s0 {
                let cs = s0;
                let ce = if e < e0 { e } else { e0 };
                segs.push((cs, ce));
            }
        }
        for (&s, &e) in self.intervals.range(s0..e0) {
            let cs = s;
            let ce = if e < e0 { e } else { e0 };
            if cs < ce {
                segs.push((cs, ce));
            }
        }

        // クリップ済み重複区間のユニオン長を求める
        let mut cur: Option<(T, T)> = None;
        for (s, e) in segs {
            if let Some((cs, ce)) = cur {
                if s <= ce {
                    let ne = if e > ce { e } else { ce };
                    cur = Some((cs, ne));
                } else {
                    removed = removed - (ce - cs);
                    cur = Some((s, e));
                }
            } else {
                cur = Some((s, e));
            }
        }
        if let Some((cs, ce)) = cur {
            removed = removed - (ce - cs);
        }

        // 2) 実際に区間集合から [s0, e0) を削除（重複区間を分割して残りを戻す）
        let mut updates: Vec<(T, T)> = Vec::new();

        // 左にまたがる直前の1区間
        if let Some((&s, &e)) = self.intervals.range(..=s0).next_back() {
            if e > s0 {
                self.intervals.remove(&s);
                // 左側の残り
                if s < s0 {
                    updates.push((s, s0));
                }
                // 右側の残り
                if e0 < e {
                    updates.push((e0, e));
                }
            }
        }

        // 開始が [s0, e0) にある区間たち
        // ループ中に remove するので、次の開始を都度取り直す
        while let Some((&s, &e)) = self.intervals.range(s0..e0).next() {
            self.intervals.remove(&s);
            // 右側の残り
            if e0 < e {
                updates.push((e0, e));
            }
        }

        // 残すべき区間を一括挿入
        for (s, e) in updates {
            // s < e を満たす場合のみ
            if s < e {
                self.intervals.insert(s, e);
            }
        }

        removed
    }

    /// 点が含まれるかチェック（最適化版）- O(log n)
    #[inline]
    pub fn contains_point(&self, point: T) -> bool {
        self.intervals
            .range(..=point)
            .next_back()
            .map(|(_, &end)| point < end)
            .unwrap_or(false)
    }

    /// 点が含まれる区間を取得（最適化版）- O(log n)
    #[inline]
    pub fn find_containing_interval(&self, point: T) -> Option<Interval<T>> {
        self.intervals
            .range(..=point)
            .next_back()
            .and_then(|(&start, &end)| {
                if point < end {
                    Some(Interval::new(start, end))
                } else {
                    None
                }
            })
    }

    /// 点を与えたとき、その点を含む区間があればそれを、
    /// なければ右側で最も近い区間（start >= point の最小）を返す。
    /// 計算量: O(log n)
    #[inline]
    pub fn next_interval_to_right(&self, point: T) -> Option<Interval<T>> {
        // 直前の区間が point を含むならそれを返す
        if let Some((&s, &e)) = self.intervals.range(..=point).next_back() {
            if point < e {
                return Some(Interval::new(s, e));
            }
        }
        // 含まれていなければ、右側で最初に始まる区間を返す
        self.intervals
            .range(point..)
            .next()
            .map(|(&s, &e)| Interval::new(s, e))
    }

    /// 区間が含まれるかチェック - O(log n)
    #[inline]
    pub fn contains_interval(&self, target: &Interval<T>) -> bool {
        if target.is_empty() {
            return true;
        }

        self.intervals
            .range(..=target.start)
            .next_back()
            .map(|(&start, &end)| start <= target.start && target.end <= end)
            .unwrap_or(false)
    }

    /// 区間と重複があるかチェック - O(log n)
    #[inline]
    pub fn overlaps_with(&self, target: &Interval<T>) -> bool {
        self.intervals
            .range(..target.end)
            .any(|(_, &end)| end > target.start)
    }

    /// すべての区間を取得
    #[inline]
    pub fn intervals(&self) -> impl Iterator<Item = Interval<T>> + '_ {
        self.intervals
            .iter()
            .map(|(&start, &end)| Interval::new(start, end))
    }

    /// 区間の数を取得
    #[inline]
    pub fn len(&self) -> usize {
        self.intervals.len()
    }

    /// 空かどうかチェック
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    /// 全体をカバーする区間を取得
    pub fn bounding_interval(&self) -> Option<Interval<T>> {
        if self.intervals.is_empty() {
            None
        } else {
            let (&first_start, _) = self.intervals.iter().next().unwrap();
            let (_, &last_end) = self.intervals.iter().next_back().unwrap();
            Some(Interval::new(first_start, last_end))
        }
    }

    /// 区間セットをクリア
    #[inline]
    pub fn clear(&mut self) {
        self.intervals.clear();
    }
}
