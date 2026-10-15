/// 再帰 DFS
///
/// # 引数
/// - `u`: 現在の頂点
/// - `graph`: 隣接リスト形式のグラフ
/// - `visited`: 訪問済みフラグ配列
pub fn dfs(u: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[u] = true;

    for &v in &graph[u] {
        if !visited[v] {
            dfs(v, graph, visited);
        }
    }
}
