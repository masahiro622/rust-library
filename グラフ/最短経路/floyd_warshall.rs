const INF: i64 = i64::MAX / 2; // 到達不能を表す無限大の値（オーバーフロー防止のため）

/// ワーシャルフロイド法による最短経路管理構造体

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloydWarshall {
    dist: Vec<Vec<i64>>,
}

impl FloydWarshall {
    /// 隣接行列からFloydWarshallを構築する。`adj`はn×n行列で、辺が存在しない場合は`INF`を使うこと。
    pub fn new(adj: &[Vec<i64>]) -> Self {
        let n = adj.len();
        assert!(adj.iter().all(|row| row.len() == n));
        let mut dist = adj.to_vec();

        // ワーシャルフロイド法: O(n^3)
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[i][k] != INF && dist[k][j] != INF {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }

        FloydWarshall { dist }
    }

    /// 指定された頂点間の最短距離を取得
    pub fn distance(&self, u: usize, v: usize) -> Option<i64> {
        let d = self.dist[u][v];
        if d == INF {
            None
        } else {
            Some(d)
        }
    }

    /// 距離行列全体を取得
    pub fn distances(&self) -> Vec<Vec<Option<i64>>> {
        self.dist
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&d| if d == INF { None } else { Some(d) })
                    .collect()
            })
            .collect()
    }

    pub fn is_negative_cycle(&self) -> bool {
        self.dist.iter().enumerate().any(|(i, row)| row[i] < 0)
    }

    /// O(n^2) でエッジを追加し、最短経路を更新
    pub fn add_edge(&mut self, u: usize, v: usize, weight: i64) {
        let n = self.dist.len();
        if weight >= self.dist[u][v] {
            return; // 既存のエッジの方が短い場合は何も変わらない
        }

        // すべての頂点ペア (i, j) に対して、新しいエッジを経由する経路をチェック
        // i -> u -> v -> j のパス
        for i in 0..n {
            for j in 0..n {
                if self.dist[i][u] != INF && self.dist[v][j] != INF {
                    self.dist[i][j] =
                        self.dist[i][j].min(self.dist[i][u] + weight + self.dist[v][j]);
                }
            }
        }
    }
}
