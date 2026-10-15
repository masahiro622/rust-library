pub trait Semiring: Clone {
    fn zero() -> Self;
    fn one() -> Self;
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
}

pub fn topological_sort<S>(n: usize, adj: &[Vec<(usize, S)>]) -> Option<Vec<usize>> {
    let mut in_degree = vec![0; n];

    // ステップ1: 各頂点の入次数（入ってくる辺の数）をカウントする
    for u in 0..n {
        for (v, _weight) in &adj[u] {
            in_degree[*v] += 1;
        }
    }

    // ステップ2: 入次数が 0 の頂点（どこからも依存されていない頂点）をキューに入れる
    let mut queue = std::collections::VecDeque::new();
    for i in 0..n {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut order = Vec::new();

    // ステップ3: キューから取り出しながら、隣接する頂点の入次数を減らしていく
    while let Some(u) = queue.pop_front() {
        order.push(u); // 取り出した順番がトポロジカル順序になる

        for (v, _weight) in &adj[u] {
            in_degree[*v] -= 1;
            // 入次数が 0 になった（すべての前提条件がクリアされた）らキューへ
            if in_degree[*v] == 0 {
                queue.push_back(*v);
            }
        }
    }

    // ステップ4: サイクル検出
    // すべての頂点を処理できていればDAG。足りなければサイクルが存在する。
    if order.len() == n {
        Some(order)
    } else {
        None
    }
}

/// トポロジカルソートされた順序 `order` を受け取り、その順番で緩和を行います。
pub fn dag_path_search<S: Semiring>(
    n: usize,
    adj: &[Vec<(usize, S)>],
    order: &[usize],
    start: usize,
) -> Vec<S> {
    let mut dist = vec![S::zero(); n];
    dist[start] = S::one();

    // 0..n ではなく、与えられたトポロジカル順序に従って頂点 u を処理する
    for &u in order {
        let du = dist[u].clone();
        for (v, weight) in &adj[u] {
            let new_dist = du.mul(weight);
            dist[*v] = dist[*v].add(&new_dist);
        }
    }

    dist
}

// =====================================================================
// 具体的な半環の実装 (前回と同じ)
// =====================================================================
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tropical(pub u32);
impl Tropical {
    const INF: u32 = u32::MAX;
}
impl Semiring for Tropical {
    fn zero() -> Self {
        Tropical(Tropical::INF)
    }
    fn one() -> Self {
        Tropical(0)
    }
    fn add(&self, other: &Self) -> Self {
        Tropical(self.0.min(other.0))
    }
    fn mul(&self, other: &Self) -> Self {
        if self.0 == Tropical::INF || other.0 == Tropical::INF {
            Tropical(Tropical::INF)
        } else {
            Tropical(self.0 + other.0)
        }
    }
}
