// 無向グラフの lowlink 系アルゴリズム
// 橋 / 関節点 / 二辺連結成分分解は共通の `dfs_lowlink` から導出し、
// 二重頂点連結成分分解は辺スタックを使う専用の dfs で求める。
use ac_library::Dsu;
use std::collections::HashSet;

/// 隣接リストを構築する（自己ループは無視し、辺には `edges` 上のインデックスを付与する）
fn build_adj(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut adj = vec![Vec::new(); n];
    for (i, &(u, v)) in edges.iter().enumerate() {
        if u == v {
            continue;
        }
        adj[u].push((v, i));
        adj[v].push((u, i));
    }
    adj
}

/// dfs 木と lowlink を求める
///
/// 戻り値 `(ord, low, parent, parent_edge)`
/// - `ord[u]`: 発見時刻
/// - `low[u]`: lowlink 値
/// - `parent[u]`: dfs 木上の親（根なら `None`）
/// - `parent_edge[u]`: `parent[u]` との間の辺の `edges` 上のインデックス（根なら `None`）
pub fn dfs_lowlink(
    n: usize,
    edges: &[(usize, usize)],
) -> (Vec<usize>, Vec<usize>, Vec<Option<usize>>, Vec<Option<usize>>) {
    let adj = build_adj(n, edges);
    let mut ord = vec![usize::MAX; n];
    let mut low = vec![usize::MAX; n];
    let mut parent = vec![None; n];
    let mut parent_edge = vec![None; n];
    let mut time = 0;

    fn dfs(
        u: usize,
        parent_edge_id: Option<usize>,
        adj: &[Vec<(usize, usize)>],
        ord: &mut [usize],
        low: &mut [usize],
        parent: &mut [Option<usize>],
        parent_edge: &mut [Option<usize>],
        time: &mut usize,
    ) {
        ord[u] = *time;
        low[u] = *time;
        *time += 1;

        for &(v, eid) in &adj[u] {
            if Some(eid) == parent_edge_id {
                continue;
            }
            if ord[v] == usize::MAX {
                parent[v] = Some(u);
                parent_edge[v] = Some(eid);
                dfs(v, Some(eid), adj, ord, low, parent, parent_edge, time);
                low[u] = low[u].min(low[v]);
            } else {
                low[u] = low[u].min(ord[v]);
            }
        }
    }

    for u in 0..n {
        if ord[u] == usize::MAX {
            dfs(
                u,
                None,
                &adj,
                &mut ord,
                &mut low,
                &mut parent,
                &mut parent_edge,
                &mut time,
            );
        }
    }

    (ord, low, parent, parent_edge)
}

/// 橋を求める（戻り値は橋となる辺の `edges` 上のインデックス）
pub fn find_bridges(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let (ord, low, parent, parent_edge) = dfs_lowlink(n, edges);

    (0..n)
        .filter_map(|v| {
            let u = parent[v]?;
            let eid = parent_edge[v]?;
            (low[v] > ord[u]).then_some(eid)
        })
        .collect()
}

/// 関節点を求める
pub fn find_articulation_points(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let (ord, low, parent, _) = dfs_lowlink(n, edges);

    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    for v in 0..n {
        if let Some(u) = parent[v] {
            children[u].push(v);
        }
    }

    (0..n)
        .filter(|&u| match parent[u] {
            // 根: dfs 木上の子が 2 つ以上あれば関節点
            None => children[u].len() >= 2,
            // 根以外: ある子 v で ord[u] <= low[v] なら u は関節点
            Some(_) => children[u].iter().any(|&v| low[v] >= ord[u]),
        })
        .collect()
}

/// 二辺連結成分分解（橋を取り除いた Union-Find で連結成分をまとめる）
pub fn two_edge_connected_components(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let bridges: HashSet<usize> = find_bridges(n, edges).into_iter().collect();

    let mut dsu = Dsu::new(n);
    for (i, &(u, v)) in edges.iter().enumerate() {
        if u != v && !bridges.contains(&i) {
            dsu.merge(u, v);
        }
    }

    dsu.groups()
}

/// 二重頂点連結成分分解（辺スタックを用いた専用の dfs）
///
/// 戻り値: 各成分に属する辺の `edges` 上のインデックスのリスト
/// （頂点は複数の成分に属しうるため、成分は辺の集合として表す）
pub fn biconnected_components(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let adj = build_adj(n, edges);
    let mut ord = vec![usize::MAX; n];
    let mut low = vec![usize::MAX; n];
    let mut time = 0;
    let mut edge_stack: Vec<usize> = Vec::new();
    let mut components: Vec<Vec<usize>> = Vec::new();

    fn dfs(
        u: usize,
        parent_edge_id: Option<usize>,
        adj: &[Vec<(usize, usize)>],
        ord: &mut [usize],
        low: &mut [usize],
        time: &mut usize,
        edge_stack: &mut Vec<usize>,
        components: &mut Vec<Vec<usize>>,
    ) {
        ord[u] = *time;
        low[u] = *time;
        *time += 1;

        for &(v, eid) in &adj[u] {
            if Some(eid) == parent_edge_id {
                continue;
            }
            if ord[v] == usize::MAX {
                edge_stack.push(eid);
                dfs(v, Some(eid), adj, ord, low, time, edge_stack, components);
                low[u] = low[u].min(low[v]);

                // u が根、または v 側を切り離せる関節点なら、ここで 1 つの成分が確定する
                if low[v] >= ord[u] {
                    let mut comp = Vec::new();
                    while let Some(top) = edge_stack.pop() {
                        comp.push(top);
                        if top == eid {
                            break;
                        }
                    }
                    components.push(comp);
                }
            } else if ord[v] < ord[u] {
                edge_stack.push(eid);
                low[u] = low[u].min(ord[v]);
            }
        }
    }

    for s in 0..n {
        if ord[s] == usize::MAX {
            dfs(
                s,
                None,
                &adj,
                &mut ord,
                &mut low,
                &mut time,
                &mut edge_stack,
                &mut components,
            );
        }
    }

    components
}
