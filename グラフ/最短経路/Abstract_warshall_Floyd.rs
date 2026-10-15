/// 閉半環（Closed Semiring）を表すトレイト
pub trait ClosedSemiring: Clone {
    /// 和の単位元（零元：⊕操作に影響を与えない値）
    fn zero() -> Self;
    /// 積の単位元（単元：⊗操作に影響を与えない値）
    fn one() -> Self;
    /// 和演算 (a ⊕ b)
    fn add(&self, other: &Self) -> Self;
    /// 積演算 (a ⊗ b)
    fn mul(&self, other: &Self) -> Self;
    /// クリーネ閉包 (a*)
    fn star(&self) -> Self;
}

/// 閉半環上の汎用ワーシャル・フロイド法
///
/// 任意の ClosedSemiring を実装した要素を持つ隣接行列を受け取り、
/// 全点対間の「距離（またはそれに準ずる値）」を計算します。
pub fn floyd_warshall<T: ClosedSemiring>(graph: &[Vec<T>]) -> Vec<Vec<T>> {
    let n = graph.len();
    // グラフの初期状態をコピー
    let mut dist = graph.to_vec();

    for k in 0..n {
        // k番目の更新を行う際、自己ループの閉包 d[k][k]* を計算
        let closure_k = dist[k][k].star();

        // インプレース更新により、同じステップ内で更新された値を使ってしまうのを防ぐため、
        // 参照するk行目とk列目の古い状態を一時的に保持します。
        let mut col_k = Vec::with_capacity(n);
        let mut row_k = Vec::with_capacity(n);
        for i in 0..n {
            col_k.push(dist[i][k].clone());
            row_k.push(dist[k][i].clone());
        }

        for i in 0..n {
            for j in 0..n {
                // path_via_k = d[i][k] ⊗ d[k][k]* ⊗ d[k][j]
                let path_via_k = col_k[i].mul(&closure_k).mul(&row_k[j]);
                // d[i][j] = d[i][j] ⊕ path_via_k
                dist[i][j] = dist[i][j].add(&path_via_k);
            }
        }
    }

    dist
}
