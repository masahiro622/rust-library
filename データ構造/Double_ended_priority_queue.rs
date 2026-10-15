#[derive(Debug, Clone)]
pub struct IntervalHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> IntervalHeap<T> {
    /// 空のIntervalHeapを作成します
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// 指定したキャパシティでIntervalHeapを作成します
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// 要素数を返します
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// ヒープが空かどうかを判定します
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 最小値の参照を取得します ($O(1)$)
    pub fn peek_min(&self) -> Option<&T> {
        self.data.first()
    }

    /// 最大値の参照を取得します ($O(1)$)
    pub fn peek_max(&self) -> Option<&T> {
        if self.data.len() > 1 {
            self.data.get(1)
        } else {
            self.data.first()
        }
    }

    /// 要素を追加します ($O(\log N)$)
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        let mut idx = self.data.len() - 1;
        if idx == 0 {
            return;
        }

        let u = idx / 2; // 現在のノードインデックス

        // ノード内で min <= max の制約を満たすようにする
        if idx % 2 == 1 && self.data[idx] < self.data[idx - 1] {
            self.data.swap(idx, idx - 1);
            idx -= 1; // min側に移動した要素を追跡
        }

        if u == 0 {
            return; // ルートノードなら終了
        }

        let parent_u = (u - 1) / 2;
        let p_min = parent_u * 2;
        let p_max = parent_u * 2 + 1;

        // 親のminより小さければ、最小ヒープ側を上にたどる
        if self.data[idx] < self.data[p_min] {
            self.sift_up_min(idx);
        }
        // 親のmaxより大きければ、最大ヒープ側を上にたどる
        else if self.data[idx] > self.data[p_max] {
            self.sift_up_max(idx);
        }
    }

    /// 最小値を取り出します ($O(\log N)$)
    pub fn pop_min(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        if self.data.len() <= 2 {
            return Some(self.data.remove(0));
        }

        // 末尾の要素を取り出し、ルートのmin(インデックス0)と入れ替えて下に沈める
        let last = self.data.pop().unwrap();
        let min_val = std::mem::replace(&mut self.data[0], last);
        self.sift_down_min(0);

        Some(min_val)
    }

    /// 最大値を取り出します ($O(\log N)$)
    pub fn pop_max(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        if self.data.len() <= 2 {
            // 要素数が1の場合は pop()、2の場合は末尾(インデックス1)が最大値なので pop()
            return Some(self.data.pop().unwrap());
        }

        // 末尾の要素を取り出し、ルートのmax(インデックス1)と入れ替えて下に沈める
        let last = self.data.pop().unwrap();
        let max_val = std::mem::replace(&mut self.data[1], last);
        self.sift_down_max(0);

        Some(max_val)
    }

    // --- 内部ヘルパー関数 ---

    fn sift_up_min(&mut self, mut idx: usize) {
        while idx >= 2 {
            let p_min = ((idx / 2 - 1) / 2) * 2;
            if self.data[idx] < self.data[p_min] {
                self.data.swap(idx, p_min);
                idx = p_min;
            } else {
                break;
            }
        }
    }

    fn sift_up_max(&mut self, mut idx: usize) {
        while idx >= 2 {
            let p_max = ((idx / 2 - 1) / 2) * 2 + 1;
            if self.data[idx] > self.data[p_max] {
                self.data.swap(idx, p_max);
                idx = p_max;
            } else {
                break;
            }
        }
    }

    fn sift_down_min(&mut self, mut curr_node: usize) {
        loop {
            let left = 2 * curr_node + 1;
            let right = 2 * curr_node + 2;

            let left_min = left * 2;
            if left_min >= self.data.len() {
                break; // 子が存在しない
            }

            // より小さい min を持つ子ノードを見つける
            let mut min_child = left;
            let right_min = right * 2;
            if right_min < self.data.len() && self.data[right_min] < self.data[left_min] {
                min_child = right;
            }

            let curr_min = curr_node * 2;
            let target_min = min_child * 2;

            if self.data[curr_min] > self.data[target_min] {
                self.data.swap(curr_min, target_min);

                // 交換後、子ノード内で min > max になっていないかチェック
                let target_max = target_min + 1;
                if target_max < self.data.len() && self.data[target_min] > self.data[target_max] {
                    self.data.swap(target_min, target_max);
                }
                curr_node = min_child;
            } else {
                break;
            }
        }
    }

    fn sift_down_max(&mut self, mut curr_node: usize) {
        loop {
            let left = 2 * curr_node + 1;
            let right = 2 * curr_node + 2;

            let left_max = match self.get_max_idx(left) {
                Some(idx) => idx,
                None => break, // 子が存在しない
            };

            // より大きい max を持つ子ノードを見つける
            let mut max_child_max_idx = left_max;
            let mut max_child = left;

            if let Some(right_max) = self.get_max_idx(right) {
                if self.data[right_max] > self.data[left_max] {
                    max_child_max_idx = right_max;
                    max_child = right;
                }
            }

            let curr_max = curr_node * 2 + 1;

            if self.data[curr_max] < self.data[max_child_max_idx] {
                self.data.swap(curr_max, max_child_max_idx);

                // 交換後、子ノード内で min > max になっていないかチェック
                let child_min = max_child * 2;
                if self.data[child_min] > self.data[max_child_max_idx] {
                    self.data.swap(child_min, max_child_max_idx);
                }
                curr_node = max_child;
            } else {
                break;
            }
        }
    }

    /// 指定したノードの max 要素のインデックスを取得します。
    /// （ノードの要素数が1つの場合は、その唯一の要素を max とみなします）
    fn get_max_idx(&self, node: usize) -> Option<usize> {
        let min_idx = node * 2;
        let max_idx = min_idx + 1;
        if max_idx < self.data.len() {
            Some(max_idx)
        } else if min_idx < self.data.len() {
            Some(min_idx)
        } else {
            None
        }
    }
}
