/// 無向グラフのオイラー路を求める関数
///
/// # 引数
/// * `n` - 頂点数
/// * `edges` - 辺のリスト (u, v)。配列のインデックスがそのまま辺のIDとして扱われます。
///
/// # 戻り値
/// オイラー路が存在する場合は `Some((頂点の列, 辺の列))`、存在しない場合は `None` を返します。
pub fn find_undirected_eulerian_path(
    n: usize,
    edges: &[(usize, usize)],
) -> Option<(Vec<usize>, Vec<usize>)> {
    let m = edges.len();
    let mut adj = vec![vec![]; n];
    let mut degree = vec![0; n];

    // 隣接リストと次数の構築
    for (i, &(u, v)) in edges.iter().enumerate() {
        adj[u].push((v, i));
        adj[v].push((u, i));
        degree[u] += 1;
        degree[v] += 1;
    }

    let mut odd_count = 0;
    let mut start_node = 0;
    let mut any_edge_node = 0;

    // 次数のチェックと始点の決定
    for i in 0..n {
        if degree[i] % 2 != 0 {
            odd_count += 1;
            start_node = i; // 奇点のいずれかを始点にする
        }
        if degree[i] > 0 {
            any_edge_node = i;
        }
    }

    // オイラー路が存在するための次数の条件
    if odd_count != 0 && odd_count != 2 {
        return None;
    }

    // すべての頂点の次数が偶数の場合は、辺を持つ任意の頂点からスタートする
    if odd_count == 0 {
        start_node = any_edge_node;
    }

    // Hierholzerのアルゴリズム (DFSベース)
    let mut used = vec![false; m];
    let mut stack = vec![(start_node, usize::MAX)];
    let mut path_v = vec![];
    let mut path_e = vec![];

    while let Some(&(u, in_edge)) = stack.last() {
        let mut moved = false;

        while let Some((v, e_idx)) = adj[u].pop() {
            if !used[e_idx] {
                used[e_idx] = true;
                stack.push((v, e_idx));
                moved = true;
                break;
            }
        }

        // 進める辺が無くなった（行き止まり）場合
        if !moved {
            stack.pop();
            path_v.push(u);
            if in_edge != usize::MAX {
                path_e.push(in_edge);
            }
        }
    }

    // すべての辺を通れなかった場合（グラフが非連結など）は None
    if path_e.len() != m {
        return None;
    }

    Some((path_v, path_e))
}
