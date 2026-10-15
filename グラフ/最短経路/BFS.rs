/// グラフに対してBFS（幅優先探索）を実行し、始点からの最短距離を計算する
///
/// # 引数
/// * `graph` - 隣接リスト表現のグラフ
/// * `start` - 探索の始点（または複数始点）
///
/// # 戻り値
/// 各頂点への最短距離。到達不可能な場合は `None`
///
/// # Panics
/// `start` が範囲外の場合にパニックする
pub fn graph_bfs(graph: &[Vec<usize>], start: usize) -> Vec<Option<usize>> {
    let n = graph.len();
    assert!(start < n, "start index out of bounds");

    let mut dist = vec![None; n];
    let mut queue = VecDeque::with_capacity(n);

    dist[start] = Some(0);
    queue.push_back(start);

    while let Some(u) = queue.pop_front() {
        for &v in &graph[u] {
            if dist[v].is_none() {
                dist[v] = Some(dist[u].unwrap() + 1);
                queue.push_back(v);
            }
        }
    }

    dist
}
