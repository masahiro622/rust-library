use std::cmp::Ordering;
use std::usize;

fn main() {
    // 頂点数
    let n = 1;

    // 辺を定義（src, dst, weight）
    let edges = vec![Edge {
        src: 0,
        dst: 1,
        weight: -1,
    }];

    // 始点を 0 としてベルマンフォード法を実行
    match bellman_ford(n, &edges, 0) {
        Ok(dist) => {
            for (i, d) in dist.iter().enumerate() {
                if *d == i64::MAX {
                    println!("頂点 {}: INF", i);
                } else {
                    println!("頂点 {}: {}", i, d);
                }
            }
        }
        Err(e) => println!("エラー: {}", e),
    }
}

/// 辺を表す構造体
#[derive(Clone)]
pub struct Edge {
    pub src: usize,
    pub dst: usize,
    pub weight: i64,
}

/// ベルマンフォード法を用いて単一始点最短経路を求める関数
pub fn bellman_ford(n: usize, edges: &[Edge], start: usize) -> Option<Vec<i64>> {
    let mut dist = vec![i64::MAX; n]; // 初期化：すべて無限大
    dist[start] = 0; // 始点の距離は0

    // n-1 回繰り返す
    for _ in 0..n - 1 {
        // 各辺を更新
        for edge in edges {
            let u = edge.src;
            let v = edge.dst;
            let weight = edge.weight;

            if dist[u] != i64::MAX && dist[u] + weight < dist[v] {
                dist[v] = dist[u] + weight;
            }
        }
    }

    // 負の閉路の検出
    for edge in edges {
        let u = edge.src;
        let v = edge.dst;
        let weight = edge.weight;

        if dist[u] != i64::MAX && dist[u] + weight < dist[v] {
            return None;
        }
    }

    Some(dist)
}
