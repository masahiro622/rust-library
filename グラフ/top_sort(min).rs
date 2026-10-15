use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// 辞書順最小のトポロジカルソート（DAG）
/// `graph[i]` は頂点 i から出ている辺の行き先リスト
/// サイクルがある場合は `None` を返す
pub fn lex_smallest_topo_sort(graph: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let n = graph.len();
    let mut in_degree = vec![0; n];

    for edges in graph {
        for &to in edges {
            in_degree[to] += 1;
        }
    }

    // BinaryHeap で最小値優先にするため Reverse を使う
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        if in_degree[i] == 0 {
            heap.push(Reverse(i));
        }
    }

    let mut order = vec![];
    while let Some(Reverse(v)) = heap.pop() {
        order.push(v);
        for &u in &graph[v] {
            in_degree[u] -= 1;
            if in_degree[u] == 0 {
                heap.push(Reverse(u));
            }
        }
    }

    if order.len() == n {
        Some(order)
    } else {
        None
    }
}
