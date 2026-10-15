use std::cmp::Reverse;
use std::ops::AddAssign;

#[derive(Clone, Debug)]
pub struct Edge<T> {
    pub u: usize,
    pub v: usize,
    pub cost: T,
    pub id: usize,
}

/// 全域木の最適化方向
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeObjective {
    /// 最小全域木
    Minimum,
    /// 最大全域木
    Maximum,
}

/// クラスカル法を扱いやすくするビルダー構造体
pub struct KruskalBuilder<T> {
    n: usize,
    edges: Vec<Edge<T>>,
}

impl<T> KruskalBuilder<T>
where
    T: Copy + Ord + Default + AddAssign,
{
    /// 頂点数を指定して初期化
    pub fn new(n: usize) -> Self {
        Self { n, edges: vec![] }
    }

    /// 辺を追加（idは自動で追加順に採番されます）
    pub fn add_edge(&mut self, u: usize, v: usize, cost: T) {
        let id = self.edges.len();
        self.edges.push(Edge { u, v, cost, id });
    }

    /// クラスカル法を実行し、(総コスト, 使用した辺のIDリスト) を返す
    pub fn build(mut self, objective: TreeObjective) -> (T, Vec<usize>) {
        // 目的によってソート順を切り替える
        match objective {
            TreeObjective::Minimum => self.edges.sort_unstable_by_key(|e| e.cost),
            TreeObjective::Maximum => self.edges.sort_unstable_by_key(|e| Reverse(e.cost)),
        }

        let mut uf = ac_library::Dsu::new(self.n);
        let mut total_cost = T::default(); // Tのデフォルト値（数値なら0）で初期化
        let mut tree_edge_ids = vec![];

        for edge in self.edges {
            if !uf.same(edge.u, edge.v) {
                uf.merge(edge.u, edge.v);
                total_cost += edge.cost;
                tree_edge_ids.push(edge.id);
            }
        }

        (total_cost, tree_edge_ids)
    }
}
