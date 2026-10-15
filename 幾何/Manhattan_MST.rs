use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub id: usize,
}

pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub cost: isize,
    pub id: usize,
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return false;
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x;
        self.size[x] += self.size[y];
        true
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

/// クラスカル法：最小全域木の総コストを返す
///
/// # 引数
/// - `n`: 頂点数
/// - `edges`: 辺のリスト
///
/// # 戻り値
/// - 最小全域木の重みの総和
pub fn kruskal(n: usize, mut edges: Vec<Edge>) -> (isize, Vec<usize>) {
    edges.sort_unstable_by_key(|e| e.cost);
    // 重みで昇順ソート
    // edges.sort_unstable_by_key(|e| Reverse(e.cost));
    //  降順（最大全域木）は上の行の代わりにこれを使う
    let mut uf = UnionFind::new(n);
    let mut total = 0;
    let mut tree = vec![];
    for edge in edges {
        if uf.unite(edge.u, edge.v) {
            total += edge.cost;
            tree.push(edge.id);
        }
    }
    tree.sort_unstable();
    (total, tree)
}

pub fn manhattan_mst(points: &[Point]) -> Vec<Edge> {
    let mut edges = vec![];
    let mut pts = points.to_vec();
    let mut edge_id = 0;

    for &(sx, sy) in &[(1, 1), (1, -1), (-1, 1), (-1, -1)] {
        for p in &mut pts {
            p.x *= sx;
            p.y *= sy;
        }

        pts.sort_by_key(|p| (p.x + p.y, p.x));

        let mut map = BTreeMap::new(); // key: y - x, value: (x, id, y)

        for &p in &pts {
            let key = p.y - p.x;
            if let Some(&(_, qid, qy)) = map.range(..=key).next_back() {
                let cost = (p.x - points[qid].x).abs() + (p.y - qy).abs();
                edges.push(Edge {
                    u: p.id,
                    v: qid,
                    cost: cost as isize,
                    id: edge_id,
                });
                edge_id += 1;
            }
            map.insert(key, (p.x, p.id, p.y));
        }

        // 戻す
        for p in &mut pts {
            p.x *= sx;
            p.y *= sy;
        }
    }

    edges
}
