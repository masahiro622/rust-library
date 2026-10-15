pub struct Tree {
    pub n: usize,
    pub g: Vec<Vec<(usize, i64)>>, // (to, weight)
    pub parent: Vec<Vec<usize>>,
    pub depth: Vec<usize>,
    pub dist: Vec<i64>, // 根からの距離
    pub log_n: usize,
}

impl Tree {
    pub fn new(n: usize) -> Self {
        let log_n = n.ilog(2) as usize + 1;
        Tree {
            n,
            g: vec![vec![]; n],
            parent: vec![vec![usize::MAX; log_n]; n],
            depth: vec![0; n],
            dist: vec![0; n],
            log_n,
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: i64) {
        self.g[u].push((v, w));
        self.g[v].push((u, w));
    }

    pub fn build(&mut self, root: usize) {
        self.dfs(root, usize::MAX, 0, 0);
        for k in 0..self.log_n - 1 {
            for v in 0..self.n {
                if self.parent[v][k] != usize::MAX {
                    self.parent[v][k + 1] = self.parent[self.parent[v][k]][k];
                }
            }
        }
    }

    fn dfs(&mut self, v: usize, p: usize, d: usize, cost: i64) {
        self.parent[v][0] = p;
        self.depth[v] = d;
        self.dist[v] = cost;
        let neighbors = self.g[v].clone();
        for (nv, w) in neighbors {
            if nv != p {
                self.dfs(nv, v, d + 1, cost + w);
            }
        }
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        for k in 0..self.log_n {
            if ((self.depth[v] - self.depth[u]) >> k) & 1 == 1 {
                v = self.parent[v][k];
            }
        }
        if u == v {
            return u;
        }
        for k in (0..self.log_n).rev() {
            if self.parent[u][k] != self.parent[v][k] {
                u = self.parent[u][k];
                v = self.parent[v][k];
            }
        }
        self.parent[u][0]
    }

    pub fn dist_between(&self, u: usize, v: usize) -> i64 {
        let lca = self.lca(u, v);
        self.dist[u] + self.dist[v] - 2 * self.dist[lca]
    }
}
