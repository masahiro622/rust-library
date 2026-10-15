#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MinWeight(pub u64);

impl PathWeight for MinWeight {
    fn start() -> Self {
        MinWeight(0)
    }
    fn infinity() -> Self {
        MinWeight(u64::MAX)
    }
    fn accumulate(self, edge: Self) -> Self {
        MinWeight(self.0.saturating_add(edge.0))
    }
    fn priority_cmp(&self, other: &Self) -> Ordering {
        // 値が「小さい」方が優先度が高い (other.cmp(self) と逆にする)
        other.0.cmp(&self.0)
    }
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// 経路の重みと評価を抽象化するトレイト
pub trait PathWeight: Copy + PartialEq {
    /// スタート地点の距離（例：0）
    fn start() -> Self;

    /// 未到達ノードの距離（例：∞ または -∞）
    fn infinity() -> Self;

    /// 距離の累積（例：加算）
    fn accumulate(self, edge: Self) -> Self;

    /// 優先度の比較。self の方が「より良い経路」であれば Ordering::Greater を返す。
    /// （BinaryHeap が最大ヒープであることを利用するため）
    fn priority_cmp(&self, other: &Self) -> Ordering;
}

/// BinaryHeap に入れるための状態構造体
#[derive(Copy, Clone, PartialEq)]
struct State<W: PathWeight> {
    weight: W,
    node: usize,
}

impl<W: PathWeight> Eq for State<W> {}

// State に対して、W::priority_cmp を使って Ord を実装する
impl<W: PathWeight> PartialOrd for State<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<W: PathWeight> Ord for State<W> {
    fn cmp(&self, other: &Self) -> Ordering {
        // 優先度が高い（Greater）ものがヒープの上に来る
        self.weight
            .priority_cmp(&other.weight)
            // 同値の場合はノード番号で比較（順序の安定性のため）
            .then_with(|| self.node.cmp(&other.node))
    }
}

pub fn dijkstra<W: PathWeight>(graph: &[Vec<(usize, W)>], start: usize) -> Vec<W> {
    let mut dist = vec![W::infinity(); graph.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = W::start();
    heap.push(State {
        weight: W::start(),
        node: start,
    });

    while let Some(State { weight, node }) = heap.pop() {
        // 取り出した経路が、現在記録されている経路より「劣る（Less）」ならスキップ
        if weight.priority_cmp(&dist[node]) == Ordering::Less {
            continue;
        }

        for &(next_node, edge_weight) in &graph[node] {
            let next_weight = weight.accumulate(edge_weight);

            // 新しい経路の方が「優れている（Greater）」なら更新してキューへ
            if next_weight.priority_cmp(&dist[next_node]) == Ordering::Greater {
                dist[next_node] = next_weight;
                heap.push(State {
                    weight: next_weight,
                    node: next_node,
                });
            }
        }
    }

    dist
}
