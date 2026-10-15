/// 二部グラフの最大マッチングを求めるライブラリ (Hopcroft-Karp法)
pub struct BipartiteMatching {
    left_size: usize,
    right_size: usize,
    graph: Vec<Vec<usize>>, // 左側の頂点から右側の頂点への隣接リスト
}

impl BipartiteMatching {
    /// 左右の頂点数を指定して初期化する
    /// `left_size`: 左側の頂点数 (インデックスは 0 ~ left_size - 1)
    /// `right_size`: 右側の頂点数 (インデックスは 0 ~ right_size - 1)
    pub fn new(left_size: usize, right_size: usize) -> Self {
        Self {
            left_size,
            right_size,
            graph: vec![vec![]; left_size],
        }
    }

    /// 左側の頂点 `u` から右側の頂点 `v` への辺を追加する
    pub fn add_edge(&mut self, u: usize, v: usize) {
        assert!(
            u < self.left_size && v < self.right_size,
            "頂点インデックスが範囲外です"
        );
        self.graph[u].push(v);
    }

    /// 最大マッチングを計算する
    /// 戻り値: (マッチングのサイズ, マッチングされた辺のリスト `(u, v)`)
    pub fn max_matching(&self) -> (usize, Vec<(usize, usize)>) {
        let mut match_u = vec![usize::MAX; self.left_size];
        let mut match_v = vec![usize::MAX; self.right_size];
        let mut dist = vec![usize::MAX; self.left_size];
        let mut matching = 0;

        // 増加パスが見つかる限り繰り返す
        while self.bfs(&match_u, &match_v, &mut dist) {
            for u in 0..self.left_size {
                if match_u[u] == usize::MAX && self.dfs(u, &mut match_u, &mut match_v, &mut dist) {
                    matching += 1;
                }
            }
        }

        // マッチング結果の復元
        let edges = match_u
            .iter()
            .enumerate()
            .filter(|&(_, &v)| v != usize::MAX)
            .map(|(u, &v)| (u, v))
            .collect();

        (matching, edges)
    }

    // BFSで増加パスの最短距離を計算する
    fn bfs(&self, match_u: &[usize], match_v: &[usize], dist: &mut [usize]) -> bool {
        let mut queue = std::collections::VecDeque::new();

        for u in 0..self.left_size {
            if match_u[u] == usize::MAX {
                dist[u] = 0;
                queue.push_back(u);
            } else {
                dist[u] = usize::MAX;
            }
        }

        let mut reached_unmatched = false;

        while let Some(u) = queue.pop_front() {
            if dist[u] != usize::MAX {
                for &v in &self.graph[u] {
                    if match_v[v] == usize::MAX {
                        reached_unmatched = true;
                    } else {
                        let next_u = match_v[v];
                        if dist[next_u] == usize::MAX {
                            dist[next_u] = dist[u] + 1;
                            queue.push_back(next_u);
                        }
                    }
                }
            }
        }
        reached_unmatched
    }

    // DFSで増加パスを見つける
    fn dfs(
        &self,
        u: usize,
        match_u: &mut [usize],
        match_v: &mut [usize],
        dist: &mut [usize],
    ) -> bool {
        for &v in &self.graph[u] {
            let next_u = match_v[v];
            if next_u == usize::MAX
                || (dist[next_u] == dist[u] + 1 && self.dfs(next_u, match_u, match_v, dist))
            {
                match_v[v] = u;
                match_u[u] = v;
                return true;
            }
        }
        // パスが見つからなかった頂点は距離を無限大にして再探索を防ぐ
        dist[u] = usize::MAX;
        false
    }
}
