/// 無向グラフと連結成分のリストから、連結成分をまとめた新しいグラフを生成する
///
/// # 引数
/// * `n` - 元のグラフの頂点数
/// * `edges` - 元のグラフの辺のリスト (u, v)
/// * `components` - 連結成分のリスト。各要素は同じ成分に属する頂点IDのVec。
///
/// # 戻り値
/// (新しいグラフの隣接リスト, 元の頂点ID -> 新しい頂点IDの配列)
fn build_condensed_graph(
    n: usize,
    edges: &[(usize, usize)],
    components: &[Vec<usize>],
) -> (Vec<Vec<usize>>, Vec<usize>) {
    let new_v_count = components.len();

    // 元の頂点IDから新しい頂点IDへのマッピング
    // 未割り当てを usize::MAX で初期化しておく
    let mut original_to_new = vec![usize::MAX; n];

    // 1. 各頂点が属する連結成分のインデックスを、新しい頂点IDとする
    for (new_id, comp) in components.iter().enumerate() {
        for &v in comp {
            original_to_new[v] = new_id;
        }
    }

    // 2. 新しいグラフを構築する
    let mut new_graph = vec![vec![]; new_v_count];

    for &(u, v) in edges {
        let nu = original_to_new[u];
        let nv = original_to_new[v];

        // 念のため components に含まれていなかった頂点（usize::MAX）の辺を弾く処理と、
        // 同じ連結成分内の辺（自己ループ）を無視する処理
        if nu != usize::MAX && nv != usize::MAX && nu != nv {
            new_graph[nu].push(nv);
            new_graph[nv].push(nu);
        }
    }

    // 3. 多重辺（異なる成分間に複数の辺があった場合）を削除する
    for adj in &mut new_graph {
        adj.sort_unstable();
        adj.dedup();
    }

    (new_graph, original_to_new)
}
