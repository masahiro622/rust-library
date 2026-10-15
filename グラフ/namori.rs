/// N頂点N辺の連結無向グラフから閉路に含まれる頂点のリストを返す
///
/// * `n`: 頂点数
/// * `edges`: 辺のリスト (u, v) ※0-indexedを想定
fn find_cycle(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut adj = vec![vec![]; n];
    let mut degree = vec![0; n];

    // グラフの構築と次数のカウント
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
        degree[u] += 1;
        degree[v] += 1;
    }

    let mut stack = vec![];

    // 次数が1の頂点（葉）をスタックに追加
    for i in 0..n {
        if degree[i] == 1 {
            stack.push(i);
        }
    }

    // 葉から順番に削っていく
    while let Some(u) = stack.pop() {
        degree[u] -= 1; // 頂点uを取り除く（次数を減らす）

        for &v in &adj[u] {
            if degree[v] > 0 {
                degree[v] -= 1; // 隣接する頂点の次数も減らす

                // 新たに葉になった頂点をキューに追加
                if degree[v] == 1 {
                    stack.push(v);
                }
            }
        }
    }

    // 最終的に次数が1より大きい（削られなかった）頂点が閉路を構成する
    let mut cycle_nodes = Vec::new();
    for i in 0..n {
        if degree[i] > 1 {
            cycle_nodes.push(i);
        }
    }

    cycle_nodes
}
