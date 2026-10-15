/// グラフが二部グラフであるか判定し、二部グラフであれば2つの独立集合に分割して返します。
///
/// * `graph` - 隣接リスト表現の無向グラフ（頂点番号は 0 から n-1 を想定）
/// * 戻り値 - 二部グラフなら `Some((集合1, 集合2))`、そうでなければ `None`
pub fn check_bipartite_and_split(graph: &[Vec<usize>]) -> Option<(Vec<usize>, Vec<usize>)> {
    let n = graph.len();
    // 各頂点の色を保持。None: 未訪問, Some(false): 色A, Some(true): 色B
    let mut colors: Vec<Option<bool>> = vec![None; n];

    // 非連結グラフの可能性も考慮し、全頂点を順にチェックする
    for start_node in 0..n {
        // まだ色を塗られていない（訪問されていない）頂点から探索を開始
        if colors[start_node].is_none() {
            let mut stack = vec![];
            stack.push(start_node);
            colors[start_node] = Some(false); // 色Aで塗る

            while let Some(u) = stack.pop() {
                let current_color = colors[u].unwrap();

                // 頂点 u に隣接するすべての頂点 v をチェック
                for &v in &graph[u] {
                    match colors[v] {
                        Some(color) => {
                            // 隣接頂点がすでに同じ色で塗られていたら、二部グラフではない
                            if color == current_color {
                                return None;
                            }
                        }
                        None => {
                            // 未訪問の頂点は、現在の頂点と「違う色」で塗ってキューに追加
                            colors[v] = Some(!current_color);
                            stack.push(v);
                        }
                    }
                }
            }
        }
    }

    // 矛盾なくすべての頂点を塗り分けられたので、2つの集合に分割する
    let mut set_a = Vec::new();
    let mut set_b = Vec::new();

    for (node, &color) in colors.iter().enumerate() {
        if !color.unwrap() {
            set_a.push(node);
        } else {
            set_b.push(node);
        }
    }

    Some((set_a, set_b))
}
