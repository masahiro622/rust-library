use std::collections::HashMap;

/// クエリの種類を表現する列挙型
#[derive(Clone, Copy)]
pub enum Query {
    Link(usize, usize),
    Cut(usize, usize),
    Add(usize, u64),
    Sum(usize),
}

/// オフライン動的連結性と成分の和を管理する構造体
pub struct OfflineDynamicConnectivity {
    n: usize,
    initial_a: Vec<u64>,
    queries: Vec<Query>,
}

impl OfflineDynamicConnectivity {
    /// n: 頂点数, initial_a: 頂点の初期値の配列
    pub fn new(n: usize, initial_a: Vec<u64>) -> Self {
        Self {
            n,
            initial_a,
            queries: Vec::new(),
        }
    }

    pub fn add_link(&mut self, u: usize, v: usize) {
        self.queries.push(Query::Link(u, v));
    }

    pub fn add_cut(&mut self, u: usize, v: usize) {
        self.queries.push(Query::Cut(u, v));
    }

    pub fn add_add(&mut self, v: usize, x: u64) {
        self.queries.push(Query::Add(v, x));
    }

    pub fn add_sum(&mut self, v: usize) {
        self.queries.push(Query::Sum(v));
    }

    /// すべてのクエリをオフラインで処理し、Sumクエリの答えの配列を返す
    pub fn solve(&self) -> Vec<u64> {
        let q = self.queries.len();
        if q == 0 {
            return Vec::new();
        }

        let mut active_edges = HashMap::new();
        let mut edge_intervals = Vec::new();

        // クエリから各辺の存在区間 [l, r) を計算
        for (i, query) in self.queries.iter().enumerate() {
            match query {
                Query::Link(u, v) => {
                    let min_v = *u.min(v);
                    let max_v = *u.max(v);
                    active_edges.insert((min_v, max_v), i);
                }
                Query::Cut(u, v) => {
                    let min_v = *u.min(v);
                    let max_v = *u.max(v);
                    if let Some(&start_time) = active_edges.get(&(min_v, max_v)) {
                        edge_intervals.push((min_v, max_v, start_time, i));
                        active_edges.remove(&(min_v, max_v));
                    }
                }
                _ => {}
            }
        }

        // 最後まで残っている辺の処理
        for (&(u, v), &start_time) in &active_edges {
            edge_intervals.push((u, v, start_time, q));
        }

        // セグメント木のサイズ（q 以上の最小の2のべき乗）
        let mut cap = 1;
        while cap < q {
            cap *= 2;
        }
        let mut edges = vec![Vec::new(); cap * 2];

        // セグメント木に辺を登録
        for (u, v, ql, qr) in edge_intervals {
            Self::add_edge_to_tree(1, 0, cap, ql, qr, u, v, &mut edges);
        }

        let mut uf = UndoableUnionFind::new(self.n, &self.initial_a);
        let mut ans = Vec::new();

        // 時間軸セグメント木上をDFS
        Self::dfs(1, 0, cap, &edges, &mut uf, &self.queries, &mut ans);

        ans
    }

    fn add_edge_to_tree(
        node: usize,
        l: usize,
        r: usize,
        ql: usize,
        qr: usize,
        u: usize,
        v: usize,
        edges: &mut [Vec<(usize, usize)>],
    ) {
        if qr <= l || r <= ql {
            return;
        }
        if ql <= l && r <= qr {
            edges[node].push((u, v));
            return;
        }
        let mid = l + (r - l) / 2;
        Self::add_edge_to_tree(node * 2, l, mid, ql, qr, u, v, edges);
        Self::add_edge_to_tree(node * 2 + 1, mid, r, ql, qr, u, v, edges);
    }

    fn dfs(
        node: usize,
        l: usize,
        r: usize,
        edges: &[Vec<(usize, usize)>],
        uf: &mut UndoableUnionFind,
        queries: &[Query],
        ans: &mut Vec<u64>,
    ) {
        let mut added = 0;
        // このノード区間に割り当てられた辺を追加
        for &(u, v) in &edges[node] {
            if uf.union(u, v) {
                added += 1;
            }
        }

        if l + 1 == r {
            // 葉に到達（時刻 l）
            if l < queries.len() {
                match queries[l] {
                    Query::Add(v, x) => uf.add_value(v, x),
                    Query::Sum(v) => ans.push(uf.get_sum(v)),
                    _ => {}
                }
            }
        } else {
            let mid = l + (r - l) / 2;
            Self::dfs(node * 2, l, mid, edges, uf, queries, ans);
            Self::dfs(node * 2 + 1, mid, r, edges, uf, queries, ans);
        }

        // 追加した辺をロールバック
        for _ in 0..added {
            uf.undo();
        }
    }
}

/// 経路圧縮をせず、ロールバックと成分和の管理ができるUnion-Find
struct UndoableUnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    sum: Vec<u64>,
    history: Vec<(usize, usize)>,
}

impl UndoableUnionFind {
    fn new(n: usize, initial_a: &[u64]) -> Self {
        Self {
            parent: vec![!0; n],
            size: vec![1; n],
            sum: initial_a.to_vec(),
            history: Vec::new(),
        }
    }

    fn find(&self, mut u: usize) -> usize {
        while self.parent[u] != !0 {
            u = self.parent[u];
        }
        u
    }

    fn union(&mut self, u: usize, v: usize) -> bool {
        let mut r_u = self.find(u);
        let mut r_v = self.find(v);
        if r_u == r_v {
            return false;
        }
        // Size によるマージ
        if self.size[r_u] < self.size[r_v] {
            std::mem::swap(&mut r_u, &mut r_v);
        }
        self.parent[r_v] = r_u;
        self.size[r_u] += self.size[r_v];

        // 子になる r_v の和を親の r_u に足す
        self.sum[r_u] += self.sum[r_v];

        self.history.push((r_u, r_v));
        true
    }

    fn undo(&mut self) {
        if let Some((r_u, r_v)) = self.history.pop() {
            let sub_val = self.sum[r_v];
            self.parent[r_v] = !0;
            self.size[r_u] -= self.size[r_v];

            // Undo時は r_u が既に別のノードの子になっている可能性があるため、
            // 現在の根まで遡って減少分(sub_val)を引く
            let mut curr = r_u;
            while curr != !0 {
                self.sum[curr] -= sub_val;
                curr = self.parent[curr];
            }
        }
    }

    fn add_value(&mut self, u: usize, x: u64) {
        // 自身から現在の根まですべての sum に加算を伝播
        let mut curr = u;
        while curr != !0 {
            self.sum[curr] += x;
            curr = self.parent[curr];
        }
    }

    fn get_sum(&self, u: usize) -> u64 {
        let r = self.find(u);
        self.sum[r]
    }
}
