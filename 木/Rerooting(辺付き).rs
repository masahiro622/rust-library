pub struct Rerooting<M: Monoid, W> {
    _marker: std::marker::PhantomData<(M, W)>,
}

impl<M: Monoid, W> Rerooting<M, W> {
    pub fn build<F, G>(graph: &[Vec<(usize, W)>], add_edge: F, add_node: G) -> Vec<M::S>
    where
        // F: child_val: 子の部分木をマージした結果, edge_val: 辺の重み, node: 現在追加しようとしている根の頂点番号
        // G: val: 親から渡される値, node: 現在追加しようとしている根の頂点番号
        F: Fn(&M::S, &W, usize) -> M::S,
        G: Fn(&M::S, usize) -> M::S,
    {
        let n = graph.len();
        if n == 0 {
            return vec![];
        } else if n == 1 {
            return vec![add_node(&M::identity(), 0)];
        }

        // 1. VecDeque の代わりに Vec をキューとして使い、リングバッファのオーバーヘッドを無くす
        let mut order = Vec::with_capacity(n);
        let mut parent = vec![usize::MAX; n];
        order.push(0);

        let mut head = 0;
        while head < n {
            let u = order[head];
            head += 1;
            for &(v, _) in &graph[u] {
                if v != parent[u] {
                    parent[v] = u;
                    order.push(v);
                }
            }
        }

        // ボトムアップの計算 (変更なし)
        let mut dp = vec![M::identity(); n];
        for &u in order.iter().rev() {
            let mut res = M::identity();
            for &(v, ref w) in &graph[u] {
                if v != parent[u] {
                    let edge_val = add_edge(&dp[v], w, v);
                    res = M::binary_operation(&res, &edge_val);
                }
            }
            dp[u] = add_node(&res, u);
        }

        let mut ans = vec![M::identity(); n];
        let mut parent_val = vec![M::identity(); n];

        // 2. ループ内での Vec アロケーションを避けるため、外側で使い回すバッファを用意
        let mut children_vals = Vec::with_capacity(n);
        let mut left = Vec::with_capacity(n + 1);

        // トップダウンの計算
        for &u in order.iter() {
            children_vals.clear();
            left.clear();
            left.push(M::identity());

            for &(v, ref w) in &graph[u] {
                let val = if v == parent[u] {
                    add_edge(&parent_val[u], w, parent[u])
                } else {
                    add_edge(&dp[v], w, v)
                };
                children_vals.push(val);

                // left配列を動的に構築
                let next_left =
                    M::binary_operation(left.last().unwrap(), children_vals.last().unwrap());
                left.push(next_left);
            }

            ans[u] = add_node(left.last().unwrap(), u);

            // 3. right 配列を排除し、逆順に走査しながら変数(suffix)で右からの累積和を管理する
            let mut suffix = M::identity();
            for (idx, &(v, _)) in graph[u].iter().enumerate().rev() {
                if v != parent[u] {
                    let val_without_v = M::binary_operation(&left[idx], &suffix);
                    parent_val[v] = add_node(&val_without_v, u);
                }
                suffix = M::binary_operation(&children_vals[idx], &suffix);
            }
        }

        ans
    }
}
