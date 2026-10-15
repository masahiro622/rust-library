fn count_euler_circuits_fixed_e0(n: usize, edges: &[(usize, usize)]) -> Mint {
    let m = edges.len();
    let mut indeg = vec![0usize; n];
    let mut outdeg = vec![0usize; n];
    let mut g = vec![Vec::<usize>::new(); n];
    let mut rg = vec![Vec::<usize>::new(); n];

    for &(u, v) in edges {
        outdeg[u] += 1;
        indeg[v] += 1;
        g[u].push(v);
        rg[v].push(u);
    }

    // オイラー閉路必要条件: in=out
    if indeg.iter().zip(&outdeg).any(|(i, o)| i != o) {
        return Mint::raw(0);
    }

    let start = edges[0].0; // e0 の始点
    let active: Vec<usize> = (0..n).filter(|&v| indeg[v] + outdeg[v] > 0).collect();

    // 正方向到達
    let mut vis = vec![false; n];
    let mut dq = VecDeque::new();
    vis[start] = true;
    dq.push_back(start);
    while let Some(v) = dq.pop_front() {
        for &to in &g[v] {
            if !vis[to] {
                vis[to] = true;
                dq.push_back(to);
            }
        }
    }
    if active.iter().any(|&v| !vis[v]) {
        return Mint::raw(0);
    }

    // 逆方向到達
    vis.fill(false);
    vis[start] = true;
    dq.push_back(start);
    while let Some(v) = dq.pop_front() {
        for &to in &rg[v] {
            if !vis[to] {
                vis[to] = true;
                dq.push_back(to);
            }
        }
    }
    if active.iter().any(|&v| !vis[v]) {
        return Mint::raw(0);
    }

    // active 頂点に圧縮
    let mut id = vec![usize::MAX; n];
    for (i, &v) in active.iter().enumerate() {
        id[v] = i;
    }
    let k = active.len();
    let mut compressed_edges = Vec::with_capacity(m);
    for &(u, v) in edges {
        compressed_edges.push((id[u], id[v]));
    }

    // τ_start
    let root = id[start];
    let tau = count_arborescences(k, &compressed_edges, root);

    // ∏ (out(v)-1)!  (active のみ)
    let mut fact = vec![Mint::raw(1); m + 1];
    for i in 1..=m {
        fact[i] = fact[i - 1] * Mint::new(i as u32);
    }

    let mut prod = Mint::raw(1);
    for &v in &active {
        // active なら outdeg[v] >= 1 (in=out かつ deg>0 のため)
        prod *= fact[outdeg[v] - 1];
    }

    tau * prod
}

/// 根を `r` とする有向全域木の個数（mod 998244353）
fn count_arborescences(n: usize, edges: &[(usize, usize)], r: usize) -> Mint {
    let mut lap = vec![vec![Mint::raw(0); n]; n];
    for &(u, v) in edges {
        if u == v {
            continue; // 自己ループは除外
        }
        lap[v][v] += 1;
        lap[v][u] -= 1;
    }

    // r行r列を除去
    let mut reduced = vec![vec![Mint::raw(0); n - 1]; n - 1];
    for i in 0..n {
        if i == r {
            continue;
        }
        for j in 0..n {
            if j == r {
                continue;
            }
            let ii = if i < r { i } else { i - 1 };
            let jj = if j < r { j } else { j - 1 };
            reduced[ii][jj] = lap[i][j];
        }
    }

    determinant_mod(reduced)
}

/// グラフの全域木の個数を数える関数
/// `n` : 頂点数
/// `edges` : 辺のリスト（多重辺可、自己ループ無視）
fn count_spanning_trees(n: usize, edges: &[(usize, usize)]) -> Mint {
    let mut laplacian = vec![vec![Mint::raw(0); n]; n];
    for &(u, v) in edges {
        if u == v {
            continue; // 自己ループは無視
        }
        laplacian[u][u] += 1;
        laplacian[v][v] += 1;
        laplacian[u][v] -= 1;
        laplacian[v][u] -= 1;
    }

    let mut reduced = vec![vec![Mint::raw(0); n - 1]; n - 1];
    for i in 1..n {
        for j in 1..n {
            reduced[i - 1][j - 1] = laplacian[i][j];
        }
    }

    determinant_mod(reduced)
}
