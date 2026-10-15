#[derive(Clone, Debug)]
pub struct Hld {
    pub depth: Vec<u32>,
    pub p_head: Vec<u32>,  // heavy pathの先頭
    pub tin: Vec<u32>,     // 頂点の訪問順序 (0-indexed)
    pub inv_tin: Vec<u32>, // tinの逆写像
    pub tout: Vec<u32>,    // 頂点の退場順序 (0-indexed)
}

impl Hld {
    const MASK: u32 = 1 << 30;

    /// n: 頂点数, edges: 辺のリスト (長さ n-1), root: 根の頂点番号
    pub fn new(n: usize, edges: Vec<(usize, usize)>, root: usize) -> Self {
        // --- XorTree の構築とボトムアップでの部分木サイズ計算 ---
        let mut deg = vec![0; n];
        let mut xor = vec![0; n];
        for (u, v) in edges {
            deg[u] += 1;
            deg[v] += 1;
            xor[u] ^= v as u32;
            xor[v] ^= u as u32;
        }

        xor[root] = root as u32;
        deg[root] = 0;

        let mut size = vec![1; n];
        let mut mx_child_sz = vec![0; n];

        for i in 0..n {
            let mut v = i;
            while deg[v] == 1 {
                deg[v] = 0;
                let p = xor[v] as usize;
                size[p] += size[v];
                mx_child_sz[p] = mx_child_sz[p].max(size[v]);
                if deg[p] == 0 {
                    break;
                }
                deg[p] -= 1;
                xor[p] ^= v as u32;
                v = p;
            }
        }

        // xor配列の最終状態が親の配列(parent)と一致する
        let mut parent = xor;

        // --- HLD の構築 ---
        let mut depth = vec![0; n];
        let mut tin = vec![0; n];
        let mut add = vec![1; n];
        let mut tout = vec![0; n];
        tout[root] = n as u32;

        let mut order = (0..n).collect::<Vec<_>>();
        order.sort_unstable_by_key(|&i| std::cmp::Reverse(size[i]));

        for &v in order.iter().skip(1) {
            let par_v = parent[v] as usize;
            depth[v] = depth[par_v] + 1;
            tin[v] = tin[par_v] + add[par_v];
            add[par_v] += size[v];
            tout[v] = tin[v] + size[v];

            if mx_child_sz[par_v] == size[v] {
                mx_child_sz[par_v] = 0;
                parent[v] = if parent[par_v] & Self::MASK == 0 {
                    parent[par_v]
                } else {
                    parent[v]
                };
            } else {
                parent[v] |= Self::MASK;
            }
        }

        let mut inv_tin = vec![0; n];
        (0..n).for_each(|i| inv_tin[tin[i] as usize] = i as u32);

        Self {
            depth,
            p_head: parent,
            tin,
            inv_tin,
            tout,
        }
    }

    fn head(&self, v: usize) -> usize {
        if self.p_head[v] & Self::MASK == 0 {
            self.p_head[v] as usize
        } else {
            v
        }
    }

    fn head_parent(&self, v: usize) -> usize {
        (self.p_head[self.head(v)] & (Self::MASK - 1)) as usize
    }

    // 最小共通祖先 (LCA)
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        while self.head(u) != self.head(v) {
            if self.depth[self.head(u)] > self.depth[self.head(v)] {
                std::mem::swap(&mut u, &mut v);
            }
            v = self.head_parent(v);
        }
        if self.tin[u] < self.tin[v] {
            u
        } else {
            v
        }
    }

    pub fn distance(&self, u: usize, v: usize) -> u32 {
        let l = self.lca(u, v);
        self.depth[u] + self.depth[v] - 2 * self.depth[l]
    }

    pub fn jump_on_tree(&self, s: usize, t: usize, k: usize) -> Option<usize> {
        let mut k = k as u32;
        let mut s = s;
        let mut t = t;
        let l = self.lca(s, t);
        let d = self.depth[s] + self.depth[t] - 2 * self.depth[l];
        if d < k {
            return None;
        } else if self.depth[s] - self.depth[l] < k {
            k = d - k;
            std::mem::swap(&mut s, &mut t);
        }
        while self.depth[s] - self.depth[self.head(s)] < k {
            k -= self.depth[s] - self.depth[self.head(s)] + 1;
            s = self.head_parent(s);
        }
        Some(self.inv_tin[(self.tin[s] - k) as usize] as usize)
    }

    pub fn reorder<T>(&self, data: Vec<T>) -> Vec<T>
    where
        T: Clone,
    {
        let n = data.len();
        let mut res = vec![data[0].clone(); n];
        for i in 0..n {
            res[self.tin[i] as usize] = data[i].clone();
        }
        res
    }

    // 頂点uからvへのパスを、heavy path上の区間の列に変換する。O(log N)
    // is_edge=true のときは辺クエリとして扱い、LCA頂点を除外する。
    pub fn query_path(&self, mut u: usize, mut v: usize, is_edge: bool) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let skip = usize::from(is_edge);
        while self.head(u) != self.head(v) {
            if self.depth[self.head(u)] > self.depth[self.head(v)] {
                let l = self.tin[self.head(u)] as usize;
                let r = self.tin[u] as usize + 1;
                if l < r {
                    res.push((l, r));
                }
                u = self.head_parent(u);
            } else {
                let l = self.tin[self.head(v)] as usize;
                let r = self.tin[v] as usize + 1;
                if l < r {
                    res.push((l, r));
                }
                v = self.head_parent(v);
            }
        }
        if self.tin[u] <= self.tin[v] {
            let l = self.tin[u] as usize + skip;
            let r = self.tin[v] as usize + 1;
            if l < r {
                res.push((l, r));
            }
        } else {
            let l = self.tin[v] as usize + skip;
            let r = self.tin[u] as usize + 1;
            if l < r {
                res.push((l, r));
            }
        }
        res
    }

    // 頂点uからvへのパスを、heavy path上の区間の列に変換する。O(log N)
    // 順向きの区間は (tin[head], tin[v]+1)、逆向きの区間は (tin[v]+1, tin[head]) で表す。
    // is_edge=true のときは辺クエリとして扱い、LCA頂点を除外する。
    pub fn query_path_non_commutative(
        &self,
        mut u: usize,
        mut v: usize,
        is_edge: bool,
    ) -> Vec<(usize, usize)> {
        let mut left = Vec::<(usize, usize)>::new();
        let mut right = Vec::<(usize, usize)>::new();
        let skip = usize::from(is_edge);

        while self.head(u) != self.head(v) {
            if self.depth[self.head(u)] > self.depth[self.head(v)] {
                // u -> head(u) は逆向き
                let l = self.tin[u] as usize + 1;
                let r = self.tin[self.head(u)] as usize;
                if l > r {
                    left.push((l, r));
                }
                u = self.head_parent(u);
            } else {
                // head(v) -> v は順向き（最後に使うので right に積む）
                let l = self.tin[self.head(v)] as usize;
                let r = self.tin[v] as usize + 1;
                if l < r {
                    right.push((l, r));
                }
                v = self.head_parent(v);
            }
        }

        // 同一 heavy path 上の最後の1区間
        if self.tin[u] <= self.tin[v] {
            let l = self.tin[u] as usize + skip;
            let r = self.tin[v] as usize + 1;
            if l < r {
                left.push((l, r)); // 順向き
            }
        } else {
            let l = self.tin[u] as usize + 1;
            let r = self.tin[v] as usize + skip;
            if l > r {
                left.push((l, r)); // 逆向き
            }
        }

        left.extend(right.into_iter().rev());
        left
    }
}
