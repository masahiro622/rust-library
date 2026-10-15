/// 隣接リストから最小全域木（MST）を O(V^2) で求める関数 （密グラフで速い）
///
/// # 引数
/// * `n` - 頂点数 (0..n)
/// * `adj` - 隣接リスト。adj[u] は (隣接頂点 v, 重み weight) のリスト
///
/// # 戻り値
/// グラフが連結であれば `Some((最小コストの合計, 最小全域木を構成する辺のリスト))` を返す。
/// 非連結の場合は `None` を返す。
/// 辺のリストは (u, v, weight) の形式。
pub fn prim_v2(n: usize, adj: &[Vec<(usize, i64)>]) -> Option<(i64, Vec<(usize, usize, i64)>)> {
    if n == 0 {
        return Some((0, vec![]));
    }

    let mut visited = vec![false; n];
    // 各頂点への最小追加コスト
    let mut min_cost = vec![i64::MAX; n];
    // 復元用：MSTにおける親頂点
    let mut parent: Vec<Option<usize>> = vec![None; n];

    // 頂点0からスタートする
    min_cost[0] = 0;

    let mut total_cost = 0;
    // 最終的に n-1 本の辺が入る
    let mut mst_edges = Vec::with_capacity(n - 1);

    for _ in 0..n {
        // 1. 未訪問の中で最小の min_cost を持つ頂点 u を探す (O(V))
        let mut min_c = i64::MAX;
        let mut u_opt = None;
        for i in 0..n {
            if !visited[i] && min_cost[i] < min_c {
                min_c = min_cost[i];
                u_opt = Some(i);
            }
        }

        // グラフが非連結の場合、到達可能な未訪問頂点がなくなる
        let u = u_opt?;

        // 頂点 u を訪問済みにする
        visited[u] = true;

        // u が始点(0)以外のとき、親が存在するので MST に辺を追加
        if let Some(p) = parent[u] {
            mst_edges.push((p, u, min_cost[u]));
            total_cost += min_cost[u];
        }

        // 2. u の隣接頂点を舐めて、min_cost と parent を更新する
        for &(v, weight) in &adj[u] {
            if !visited[v] && weight < min_cost[v] {
                min_cost[v] = weight;
                parent[v] = Some(u);
            }
        }
    }

    Some((total_cost, mst_edges))
}

fn main() {
    let n = 4;
    let mut adj = vec![vec![]; n];

    // 無向グラフの構築 (u, v, weight)
    let edges = [(0, 1, 10), (0, 2, 6), (0, 3, 5), (1, 3, 15), (2, 3, 4)];

    for &(u, v, w) in &edges {
        adj[u].push((v, w));
        adj[v].push((u, w)); // 無向グラフなので両方向に追加
    }

    match prim_v2(n, &adj) {
        Some((cost, mst)) => {
            println!("最小コスト: {}", cost);
            println!("構成される辺:");
            for (u, v, w) in mst {
                println!("  {} - {} (重み: {})", u, v, w);
            }
        }
        None => {
            println!("グラフが非連結です。");
        }
    }
}
