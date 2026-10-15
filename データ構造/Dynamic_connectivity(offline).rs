use std::collections::HashMap;

/// 履歴を保持し、状態の巻き戻し（Undo）が可能なUnion-Find
#[derive(Debug, Clone)]
pub struct RollbackUnionFind {
    parent_or_size: Vec<i32>,
    history: Vec<(usize, i32)>, // (index, 変更前の値)
}

impl RollbackUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent_or_size: vec![-1; n],
            history: vec![],
        }
    }

    /// 経路圧縮は行わない（Rollback計算量が壊れるため）
    pub fn find(&self, mut x: usize) -> usize {
        while self.parent_or_size[x] >= 0 {
            x = self.parent_or_size[x] as usize;
        }
        x
    }

    pub fn same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.find(x);
        y = self.find(y);
        if x == y {
            return false;
        }
        // サイズによるマージ（xを大きい方にする）
        if self.parent_or_size[x] > self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        // 変更前の状態を記録
        self.history.push((x, self.parent_or_size[x]));
        self.history.push((y, self.parent_or_size[y]));

        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        true
    }

    /// 現在の状態（履歴の長さ）を保存
    pub fn snapshot(&self) -> usize {
        self.history.len()
    }

    /// 指定した状態まで巻き戻す
    pub fn rollback(&mut self, state: usize) {
        while self.history.len() > state {
            let (i, val) = self.history.pop().unwrap();
            self.parent_or_size[i] = val;
        }
    }
}

/// Offline Dynamic Connectivity
/// クエリを先読みし、時間軸セグメント木を用いて処理する
pub struct OfflineDynamicConnectivity {
    n: usize,
    q: usize,
    active_edges: HashMap<(usize, usize), usize>, // 辺 -> 追加された時刻
    edge_intervals: Vec<(usize, usize, usize, usize)>, // (u, v, 開始時刻, 終了時刻)
    queries: Vec<Option<(usize, usize)>>,         // Some((u, v)) は判定クエリ
}

impl OfflineDynamicConnectivity {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            q: 0,
            active_edges: HashMap::new(),
            edge_intervals: Vec::new(),
            queries: Vec::new(),
        }
    }

    fn normalize_edge(u: usize, v: usize) -> (usize, usize) {
        if u < v { (u, v) } else { (v, u) }
    }

    /// 辺の追加（時刻が1進む）
    pub fn add_edge(&mut self, u: usize, v: usize) {
        let edge = Self::normalize_edge(u, v);
        self.active_edges.insert(edge, self.q);
        self.queries.push(None);
        self.q += 1;
    }

    /// 辺の削除（時刻が1進む）
    pub fn remove_edge(&mut self, u: usize, v: usize) {
        let edge = Self::normalize_edge(u, v);
        if let Some(start_time) = self.active_edges.remove(&edge) {
            self.edge_intervals
                .push((edge.0, edge.1, start_time, self.q));
        }
        self.queries.push(None);
        self.q += 1;
    }

    /// 連結判定クエリ（時刻が1進む）
    pub fn ask(&mut self, u: usize, v: usize) {
        self.queries.push(Some((u, v)));
        self.q += 1;
    }

    /// すべてのクエリを処理し、askに対応する解答の配列を返す
    pub fn solve(mut self) -> Vec<bool> {
        // 削除されずに最後まで残っている辺の終了時刻を q に設定
        for (edge, start_time) in self.active_edges.into_iter() {
            self.edge_intervals
                .push((edge.0, edge.1, start_time, self.q));
        }

        // セグメント木のサイズを決定（2の冪乗）
        let sz = self.q.next_power_of_two();
        let mut seg = vec![Vec::new(); 2 * sz];

        // 辺が存在する区間 [l, r) をセグメント木に登録
        for &(u, v, mut l, mut r) in &self.edge_intervals {
            l += sz;
            r += sz;
            while l < r {
                if l % 2 == 1 {
                    seg[l].push((u, v));
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    seg[r].push((u, v));
                }
                l >>= 1;
                r >>= 1;
            }
        }

        let mut uf = RollbackUnionFind::new(self.n);
        let mut ans = Vec::new();

        Self::dfs(1, &seg, &mut uf, &self.queries, sz, &mut ans);
        ans
    }

    fn dfs(
        k: usize,
        seg: &[Vec<(usize, usize)>],
        uf: &mut RollbackUnionFind,
        queries: &[Option<(usize, usize)>],
        sz: usize,
        ans: &mut Vec<bool>,
    ) {
        // 潜る前の状態を記録
        let state = uf.snapshot();

        // このノードに登録されている辺をすべて追加
        for &(u, v) in &seg[k] {
            uf.union(u, v);
        }

        if k >= sz {
            // 葉に到達した（特定の時刻 t に対応）
            let time = k - sz;
            if time < queries.len()
                && let Some((u, v)) = queries[time]
            {
                ans.push(uf.same(u, v));
            }
        } else {
            // 子ノードへ再帰的に潜る
            Self::dfs(k * 2, seg, uf, queries, sz, ans);
            Self::dfs(k * 2 + 1, seg, uf, queries, sz, ans);
        }

        // 親に戻る際に、追加した辺をすべて取り消す
        uf.rollback(state);
    }
}
