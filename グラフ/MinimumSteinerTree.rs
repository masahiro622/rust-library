use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: u64 = 1_000_000_000_000_000_000;

#[derive(Clone, Copy)]
pub struct Edge {
    pub to: usize,
    pub cost: u64,
}

/// 最小シュタイナー木のコストを計算する
///
/// * `n` - グラフの頂点数
/// * `graph` - 隣接リスト形式のグラフ
/// * `terminals` - 接続したいターミナル頂点のリスト
pub fn minimum_steiner_tree(n: usize, graph: &[Vec<Edge>], terminals: &[usize]) -> u64 {
    let k = terminals.len();
    if k == 0 {
        return 0;
    }
    if k == 1 {
        return 0; // ターミナルが1つの場合は結ぶ必要がないのでコスト0
    }

    // dp[S][v]: ターミナルの集合Sをカバーし、頂点vを根とする木の最小コスト
    let mut dp = vec![vec![INF; n]; 1 << k];

    // ターミナル頂点の初期化
    for (i, &t) in terminals.iter().enumerate() {
        dp[1 << i][t] = 0;
    }

    // ビットDP (S は 1 から 2^k - 1 まで)
    for i in 1..(1 << k) {
        // 遷移1: 集合Sを2つに分割して結合
        for v in 0..n {
            // i の部分集合を列挙するビット演算のテクニック
            let mut j = (i - 1) & i;
            while j > 0 {
                if dp[j][v] != INF && dp[i ^ j][v] != INF {
                    dp[i][v] = dp[i][v].min(dp[j][v] + dp[i ^ j][v]);
                }
                j = (j - 1) & i;
            }
        }

        // 遷移2: ダイクストラ法による辺の延長
        let mut heap = BinaryHeap::new();
        for v in 0..n {
            if dp[i][v] != INF {
                heap.push((Reverse(dp[i][v]), v));
            }
        }

        while let Some((Reverse(d), u)) = heap.pop() {
            if d > dp[i][u] {
                continue;
            }
            for edge in &graph[u] {
                let next_cost = d + edge.cost;
                if next_cost < dp[i][edge.to] {
                    dp[i][edge.to] = next_cost;
                    heap.push((Reverse(next_cost), edge.to));
                }
            }
        }
    }

    // すべてのターミナルを含む(集合 (1<<k)-1) 最小コストをすべての頂点から探す
    dp[(1 << k) - 1].iter().copied().min().unwrap_or(INF)
}
