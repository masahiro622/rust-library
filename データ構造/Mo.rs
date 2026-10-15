pub struct Mo {
    // (L, R, クエリの元のインデックス)
    queries: Vec<(usize, usize, usize)>,
}

impl Mo {
    pub fn new() -> Self {
        Mo { queries: vec![] }
    }
}

impl Default for Mo {
    fn default() -> Self {
        Mo::new()
    }
}

impl Mo {
    /// クエリ区間 [l, r) を追加
    pub fn insert(&mut self, l: usize, r: usize) {
        let idx = self.queries.len();
        self.queries.push((l, r, idx));
    }

    /// (l, r) から計算したキーでクエリをソートし、インデックス列を返す
    fn sorted_indices<F: Fn(usize, usize) -> u64>(&self, key: F) -> Vec<usize> {
        let mut keyed: Vec<(u64, usize)> = self
            .queries
            .iter()
            .map(|&(l, r, idx)| (key(l, r), idx))
            .collect();
        keyed.sort_unstable_by_key(|&(k, _)| k);
        keyed.into_iter().map(|(_, idx)| idx).collect()
    }

    /// 帯ごとに R の昇順・降順を交互にする順序（L を shift だけずらして帯を切る）
    fn odd_even_order(&self, n: usize, block_size: usize, shift: usize) -> Vec<usize> {
        self.sorted_indices(|l, r| {
            let b = (l + shift) / block_size;
            let r_key = if b.is_multiple_of(2) { r } else { n - r };
            (b * (n + 1) + r_key) as u64
        })
    }

    fn path_length(&self, order: &[usize]) -> u64 {
        let mut cur_l = 0usize;
        let mut cur_r = 0usize;
        let mut total = 0u64;

        for &idx in order {
            let (l, r, _) = self.queries[idx];
            total += cur_l.abs_diff(l) as u64 + cur_r.abs_diff(r) as u64;
            cur_l = l;
            cur_r = r;
        }

        total
    }

    fn ordered_queries(&self) -> Vec<(usize, usize, usize)> {
        if self.queries.is_empty() {
            return vec![];
        }

        let n = self
            .queries
            .iter()
            .map(|&(_, r, _)| r)
            .max()
            .unwrap()
            .max(1);
        let q = self.queries.len() as f64;
        // 移動量 Q*B/3 + N^2/B を最小化する B = sqrt(3)*N/sqrt(Q) 付近。実測で 1.5 が良好
        let block_size = ((1.5 * n as f64 / q.sqrt()).ceil() as usize).max(1);

        [
            self.odd_even_order(n, block_size, 0),
            self.odd_even_order(n, block_size, block_size / 2),
        ]
        .iter()
        .min_by_key(|order| self.path_length(order))
        .unwrap()
        .iter()
        .map(|idx| self.queries[*idx])
        .collect()
    }

    /// Mo法を実行する（状態 state を各クロージャに &mut で渡す版）
    /// 可変参照を共有するものはすべてstateにまとめて渡すこと。
    /// add, del は let で定義せずクロージャーを直接渡すこと。
    /// 候補の順序（奇偶ソート / B/2 ずらした奇偶ソート）のうち
    /// 推定移動距離が最短のものを採用して実行する。
    pub fn execute<S, AddL, AddR, DelL, DelR, Solve>(
        &self,
        state: &mut S,
        mut add_l: AddL,
        mut add_r: AddR,
        mut del_l: DelL,
        mut del_r: DelR,
        mut solve: Solve,
    ) where
        AddL: FnMut(&mut S, usize),
        AddR: FnMut(&mut S, usize),
        DelL: FnMut(&mut S, usize),
        DelR: FnMut(&mut S, usize),
        Solve: FnMut(&mut S, usize), // 第2引数はクエリの元のインデックス
    {
        let mut nl = 0;
        let mut nr = 0;

        for (l, r, idx) in self.ordered_queries() {
            // [nl, nr) から [l, r) へ区間を伸縮させる
            while nl > l {
                nl -= 1;
                add_l(state, nl);
            }
            while nr < r {
                add_r(state, nr);
                nr += 1;
            }
            while nl < l {
                del_l(state, nl);
                nl += 1;
            }
            while nr > r {
                nr -= 1;
                del_r(state, nr);
            }
            solve(state, idx);
        }
    }
}
