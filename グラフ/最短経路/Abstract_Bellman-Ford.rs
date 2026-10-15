use std::fmt::Debug;

/// 冪等半環 (Idempotent Semiring) を表すトレイト
/// 条件: a ⊕ a = a (冪等性)
pub trait IdempotentSemiring: Clone + PartialEq + Debug {
    /// 加法の単位元 (零元): 任意の a に対して a ⊕ zero = a, a ⊗ zero = zero
    fn zero() -> Self;

    /// 乗法の単位元 (壱元): 任意の a に対して a ⊗ one = a
    fn one() -> Self;

    /// 加法 (⊕): 経路の「要約」や「選択」を表す (例: min, max, OR)
    fn oplus(&self, other: &Self) -> Self;

    /// 乗法 (⊗): 経路の「拡張」や「結合」を表す (例: +, AND)
    fn otimes(&self, other: &Self) -> Self;
}

/// グラフの辺を表す構造体
#[derive(Clone, Debug)]
pub struct Edge<W> {
    pub u: usize, // 始点
    pub v: usize, // 終点
    pub w: W,     // 重み（半環の要素）
}

/// 冪等半環上で動作する汎用 Bellman-Ford アルゴリズム
///
/// 引数:
/// - n: 頂点数
/// - edges: 辺のリスト
/// - source: 始点となる頂点
///
/// 戻り値:
/// - Ok(Vec<W>): 各頂点への「距離」（半環の要素）の配列
/// - Err(&str): 負の閉路（距離を無限に更新し続ける閉路）を検出した場合
pub fn bellman_ford<W: IdempotentSemiring>(
    n: usize,
    edges: &[Edge<W>],
    source: usize,
) -> Result<Vec<W>, &'static str> {
    // 初期化: 始点は壱元(one)、それ以外は零元(zero)で埋める
    let mut dist = vec![W::zero(); n];
    dist[source] = W::one();

    // |V| - 1 回の緩和 (Relaxation)
    for _ in 0..n - 1 {
        let mut updated = false;
        for edge in edges {
            // dist[v] = dist[v] ⊕ (dist[u] ⊗ w)
            let extended_path = dist[edge.u].otimes(&edge.w);
            let relaxed = dist[edge.v].oplus(&extended_path);

            if relaxed != dist[edge.v] {
                dist[edge.v] = relaxed;
                updated = true;
            }
        }

        // 更新がなければ早期終了
        if !updated {
            break;
        }
    }

    // 閉路の検出 (さらに緩和できるかチェック)
    for edge in edges {
        let extended_path = dist[edge.u].otimes(&edge.w);
        let relaxed = dist[edge.v].oplus(&extended_path);

        if relaxed != dist[edge.v] {
            return Err("Graph contains a cycle that indefinitely relaxes distances.");
        }
    }

    Ok(dist)
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tropical(pub f64);

impl IdempotentSemiring for Tropical {
    // 零元 (到達不能): 加法(min)の単位元なので、無限大
    fn zero() -> Self {
        Tropical(f64::INFINITY)
    }

    // 壱元 (始点): 乗法(+)の単位元なので、0
    fn one() -> Self {
        Tropical(0.0)
    }

    // 加法 ⊕: 2つの経路のうち、短い方を選ぶ (min)
    fn oplus(&self, other: &Self) -> Self {
        Tropical(self.0.min(other.0))
    }

    // 乗法 ⊗: 経路を繋げて距離を足し合わせる (+)
    fn otimes(&self, other: &Self) -> Self {
        Tropical(self.0 + other.0)
    }
}
