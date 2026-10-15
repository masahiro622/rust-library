use std::collections::VecDeque;

/// 擬似コードにおける「半環（Semiring）」の定義
/// K: 半環の要素の型
pub trait Semiring: Clone + PartialEq {
    fn zero() -> Self; // 零元 (0バー)
    fn one() -> Self; // 単位元 (1バー)
    fn oplus(&self, other: &Self) -> Self; // 加法 (⊕)
    fn otimes(&self, other: &Self) -> Self; // 乗法 (⊗)
}

/// グラフのエッジ（辺）
#[derive(Clone)]
pub struct Edge<W: Semiring> {
    to: usize,
    weight: W,
}

/// 隣接リスト表現のグラフ
pub struct Graph<W: Semiring> {
    adj: Vec<Vec<Edge<W>>>,
}

impl<W: Semiring> Graph<W> {
    pub fn new(size: usize) -> Self {
        Graph {
            adj: vec![Vec::new(); size],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        self.adj[from].push(Edge { to, weight });
    }
}

/// 汎用・単一始点最短経路アルゴリズム (Generic-Single-Source-Shortest-Distance)
/// 冪等でなくても動作するが、k-closedでない場合は停止しないことに注意
pub fn generic_sssp<W: Semiring>(graph: &Graph<W>, source: usize) -> Vec<W> {
    let n = graph.adj.len();

    // 1-3行目: d (推定最短距離) と r (未処理の差分) の初期化
    let mut d = vec![W::zero(); n];
    let mut r = vec![W::zero(); n];

    // S: 処理待ちキュー (in_queue でキュー内にあるかを O(1) で判定)
    let mut queue = VecDeque::new();
    let mut in_queue = vec![false; n];

    // 始点 s の設定 (3-4行目)
    d[source] = W::one();
    r[source] = W::one();
    queue.push_back(source);
    in_queue[source] = true;

    // 5行目: while S != ∅
    while let Some(q) = queue.pop_front() {
        in_queue[q] = false; // キューから取り出された

        // 8-9行目: r' <- r[q], r[q] <- 0
        let r_prime = r[q].clone();
        r[q] = W::zero();

        // 10行目: for each e ∈ E[q]
        for edge in &graph.adj[q] {
            let next_node = edge.to;
            let weight = &edge.weight;

            // r' ⊗ w[e] (追加される新しい重み)
            let added_weight = r_prime.otimes(weight);

            // d[n[e]] ⊕ (r' ⊗ w[e]) (新しい推定値)
            let new_d = d[next_node].oplus(&added_weight);

            // 11行目: if d[n[e]] != d[n[e]] ⊕ (r' ⊗ w[e])
            if d[next_node] != new_d {
                // 12-13行目: d と r の更新
                d[next_node] = new_d;
                r[next_node] = r[next_node].oplus(&added_weight);

                // 14-15行目: if n[e] ∉ S then ENQUEUE
                if !in_queue[next_node] {
                    queue.push_back(next_node);
                    in_queue[next_node] = true;
                }
            }
        }
    }

    // 16行目: d[s] <- 1
    d[source] = W::one();

    d
}

/// 経路数を数えるための非冪等な半環（数値型をラップして実装）
#[derive(Clone, PartialEq, Debug)]
struct PathCount(u32);

impl Semiring for PathCount {
    fn zero() -> Self {
        PathCount(0)
    }
    fn one() -> Self {
        PathCount(1)
    }

    // ⊕: 経路が合流するときはパターン数を足す
    fn oplus(&self, other: &Self) -> Self {
        PathCount(self.0 + other.0)
    }
    // ⊗: 経路が直列に繋がるときはパターン数を掛ける
    fn otimes(&self, other: &Self) -> Self {
        PathCount(self.0 * other.0)
    }
}
