const MOD: i64 = 998_244_353;

pub struct WeightedUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    weight: Vec<i64>, // weight[x]: x から親への「差分」
}

impl WeightedUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            weight: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let orig = self.parent[x];
            let root = self.find(orig);
            self.weight[x] = (self.weight[x] + self.weight[orig]) % MOD;
            self.parent[x] = root;
            root
        }
    }

    pub fn diff(&mut self, x: usize) -> i64 {
        self.find(x);
        self.weight[x]
    }

    /// a[u] ≡ a[v] + w (mod MOD)
    /// ⇒ weight[u] = weight[v] + w となるようにマージ
    pub fn unite(&mut self, u: usize, v: usize, w: i64) -> bool {
        let ru = self.find(u);
        let rv = self.find(v);
        if ru == rv {
            // 差がすでに決まっていて、それと矛盾しないか検証
            let actual = (self.diff(u) - self.diff(v) + MOD) % MOD;
            return actual == w;
        }

        let w = (w + self.diff(v) - self.diff(u) + MOD) % MOD;

        // ランクに基づいてマージ
        if self.rank[ru] < self.rank[rv] {
            self.parent[ru] = rv;
            self.weight[ru] = w;
        } else {
            self.parent[rv] = ru;
            self.weight[rv] = (MOD - w) % MOD;
            if self.rank[ru] == self.rank[rv] {
                self.rank[ru] += 1;
            }
        }
        true
    }

    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }

    /// a[u] - a[v] の差分を返す（mod MOD）
    pub fn diff_between(&mut self, u: usize, v: usize) -> Option<i64> {
        if self.is_same(u, v) {
            Some((self.diff(u) - self.diff(v) + MOD) % MOD)
        } else {
            None
        }
    }

    pub fn count_roots(&mut self) -> usize {
        let n = self.parent.len();
        (0..n).filter(|&i| self.find(i) == i).count()
    }
}
