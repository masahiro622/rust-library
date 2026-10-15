/// 静的な木において、辺のリストから直接ボトムアップに各部分木の値を計算する関数
///
/// * `n`: 頂点数
/// * `edges`: 辺のリスト (0-indexed, 無向辺)
/// * `root`: 根となる頂点 (0-indexed)
/// * `apply_vertex`: 子をすべてマージした結果に、頂点自身（第一引数）の情報を付加するクロージャ
pub fn solve_tree_dp<M, F>(
    n: usize,
    edges: &[(usize, usize)],
    root: usize,
    mut apply_vertex: F,
) -> (Vec<M::S>, Vec<usize>)
where
    M: Monoid,
    F: FnMut(usize, M::S) -> M::S,
{
    if n == 0 {
        return (vec![], vec![]);
    }
    if n == 1 {
        let mut dp = vec![M::identity(); 1];
        dp[0] = apply_vertex(0, M::identity());
        return (dp, vec![usize::MAX]);
    }

    // 1. XOR木（XOR Adjacency）の構築
    // 各頂点の次数（degree）と、隣接する頂点番号のXOR総和（xor_adj）を計算
    let mut degree = vec![0; n];
    let mut xor_adj = vec![0; n];
    for &(u, v) in edges {
        degree[u] += 1;
        degree[v] += 1;
        xor_adj[u] ^= v as u32;
        xor_adj[v] ^= u as u32;
    }

    // 2. 葉から順に削ってボトムアップの順序（order）と親（parent）を確定
    let mut order = Vec::with_capacity(n - 1);
    let mut parent = vec![usize::MAX; n];
    let mut st = Vec::new(); // 葉を管理するスタック

    // 指定された root 以外の「次数が1の頂点（葉）」を初期状態としてスタックに入れる
    for u in 0..n {
        if u != root && degree[u] == 1 {
            st.push(u as u32);
        }
    }

    while let Some(u) = st.pop() {
        order.push(u);
        // uはu32なので索引用にusizeに変換
        let u_us = u as usize;
        // uの子はすべて削られているため、現在 xor_adj[u] に残っている値がそのまま「親」になる
        let p = xor_adj[u_us] as usize;
        parent[u_us] = p;

        // 親 p から自分（u）の情報を消去する
        xor_adj[p] ^= u;
        degree[p] -= 1;

        // 親が root 以外で、かつ葉（次数1）になったらスタックに追加
        if p != root && degree[p] == 1 {
            st.push(p as u32);
        }
    }

    // 3. 配るDPによるボトムアップ計算
    let mut dp = vec![M::identity(); n];

    // order はすでに「葉から根へ」向かうボトムアップの順序になっている
    for u in order.iter().map(|x| *x as usize) {
        // 現在の dp[u] には子からのマージ結果が溜まっている
        let res = std::mem::replace(&mut dp[u], M::identity());
        dp[u] = apply_vertex(u, res);

        // 自分の確定した値を親に配る（マージする）
        let p = parent[u] as usize;
        dp[p] = M::binary_operation(&dp[p], &dp[u]);
    }

    // 最後に、蓄積が終わった根（root）のDP値を確定させる
    let root_res = std::mem::replace(&mut dp[root], M::identity());
    dp[root] = apply_vertex(root, root_res);

    (dp, parent)
}
