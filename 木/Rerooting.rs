pub struct Rerooting<M>(std::marker::PhantomData<M>);

impl<M: Monoid> Rerooting<M> {
    /// edges: (u, v) の一方向、または双方向の辺リスト
    /// n: 頂点数
    pub fn build<F>(n: usize, edges: &[(usize, usize)], mut add_root: F) -> Vec<M::S>
    where
        M: Monoid,
        M::S: Clone,
        // val: 子の部分木をマージした結果, node: 現在追加しようとしている根の頂点番号
        F: FnMut(&M::S, usize) -> M::S,
    {
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![add_root(&M::identity(), 0)];
        }

        // 1. CSR形式 (フラットな隣接リスト) の構築
        let mut degree = vec![0; n];
        for &(u, v) in edges {
            degree[u] += 1;
            degree[v] += 1;
        }

        let mut head = vec![0; n + 1];
        for i in 0..n {
            head[i + 1] = head[i] + degree[i];
        }

        // 隣接頂点を1次元配列にフラットに格納
        let mut adjacency = vec![0; head[n]];
        let mut offset = head.clone();
        for &(u, v) in edges {
            adjacency[offset[u]] = v;
            offset[u] += 1;
            adjacency[offset[v]] = u;
            offset[v] += 1;
        }

        // 2. トポロジカル順序の構築
        let mut order = Vec::with_capacity(n);
        let mut parent = vec![usize::MAX; n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(0);

        while let Some(u) = q.pop_front() {
            order.push(u);
            for idx in head[u]..head[u + 1] {
                let v = adjacency[idx];
                if v != parent[u] {
                    parent[v] = u;
                    q.push_back(v);
                }
            }
        }

        // 3. ボトムアップ (DFS1)
        let mut dp = vec![M::identity(); n];
        for &u in order.iter().rev() {
            let mut res = M::identity();
            for idx in head[u]..head[u + 1] {
                let v = adjacency[idx];
                if v != parent[u] {
                    res = M::binary_operation(&res, &dp[v]);
                }
            }
            dp[u] = add_root(&res, u);
        }

        // 4. トップダウン (DFS2) - バッファを外で1回だけ確保
        let mut ans = vec![M::identity(); n];
        let mut parent_val = vec![M::identity(); n];

        // テンポラリバッファ（最大次数+1 あれば十分なので n+1 で初期化）
        let mut children_vals = vec![M::identity(); n + 1];
        let mut left = vec![M::identity(); n + 1];
        let mut right = vec![M::identity(); n + 1];

        for &u in order.iter() {
            let mut deg = 0;
            for idx in head[u]..head[u + 1] {
                let v = adjacency[idx];
                if v == parent[u] {
                    children_vals[deg] = parent_val[u].clone();
                } else {
                    children_vals[deg] = dp[v].clone();
                }
                deg += 1;
            }

            // 累積和の計算（既存の領域を上書き利用）
            left[0] = M::identity();
            for i in 0..deg {
                left[i + 1] = M::binary_operation(&left[i], &children_vals[i]);
            }

            right[deg] = M::identity();
            for i in (0..deg).rev() {
                right[i] = M::binary_operation(&right[i + 1], &children_vals[i]);
            }

            ans[u] = add_root(&left[deg], u);

            for (idx_in_child, idx) in (head[u]..head[u + 1]).enumerate() {
                let v = adjacency[idx];
                if v != parent[u] {
                    let val_without_v =
                        M::binary_operation(&left[idx_in_child], &right[idx_in_child + 1]);
                    parent_val[v] = add_root(&val_without_v, u);
                }
            }
        }

        ans
    }
}
