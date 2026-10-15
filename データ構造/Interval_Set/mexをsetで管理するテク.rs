use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct MexIntervalSet {
    // key: start, value: end. 区間は [start, end)
    intervals: BTreeMap<i64, i64>,
}

impl MexIntervalSet {
    pub fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
        }
    }

    /// MEXに特化した挿入（単一点のみ）
    pub fn insert_point(&mut self, point: i64) {
        // 1. 既に含まれているかチェック (ここが抜けていました)
        if let Some((&_start, &end)) = self.intervals.range(..=point).next_back() {
            if point < end {
                return; // 既に区間内にあるので何もしない
            }
        }

        let start = point;
        let end = point + 1;

        // 2. 左隣と結合できるかチェック
        // start(=point) の直前から始まる区間を探す
        let mut merge_start = start;
        if let Some((&prev_start, &prev_end)) = self.intervals.range(..start).next_back() {
            if prev_end == start {
                merge_start = prev_start;
                // 左隣を削除 (後でまとめて insert する)
                self.intervals.remove(&prev_start);
            }
        }

        // 3. 右隣と結合できるかチェック
        // end(=point+1) から始まる区間を探す
        let mut merge_end = end;
        // BTreeMapの仕様上、removeするとイテレータが無効になる可能性があるため、
        // 先にキーを取得してから削除判定を行う
        if let Some((&next_start, &next_end)) = self.intervals.range(end..).next() {
            if next_start == end {
                merge_end = next_end;
                // 右隣を削除
                self.intervals.remove(&next_start);
            }
        }

        // 4. 結合した区間を挿入
        self.intervals.insert(merge_start, merge_end);
    }

    /// MEXに特化した削除（単一点のみ）
    pub fn remove_point(&mut self, point: i64) {
        // point を含む区間を探す
        if let Some((&start, &end)) = self.intervals.range(..=point).next_back() {
            // 実際に区間に含まれているか確認
            if point < end {
                // 元の区間を削除
                self.intervals.remove(&start);

                // 左側が残るなら追加 [start, point)
                if start < point {
                    self.intervals.insert(start, point);
                }
                // 右側が残るなら追加 [point+1, end)
                if point + 1 < end {
                    self.intervals.insert(point + 1, end);
                }
            }
        }
    }

    /// MEX計算（最適化版）
    pub fn find_mex(&self) -> i64 {
        // 最小の区間を取得
        if let Some((&start, &end)) = self.intervals.iter().next() {
            // 0から始まっているなら、その区間の終端がMEX
            if start == 0 {
                end
            } else {
                // 0から始まっていない（例: [2, 5)）なら、MEXは 0
                0
            }
        } else {
            // 区間が一つもないなら 0
            0
        }
    }
}
