#[derive(Clone, Debug)]
struct WeightedXorTree {
    deg: Vec<u32>,
    xor: Vec<u32>,
    weight: Vec<usize>,
}

impl WeightedXorTree {
    fn new(n: usize) -> Self {
        Self {
            deg: vec![0; n],
            xor: vec![0; n],
            weight: vec![0; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, c: usize) {
        self.deg[u] += 1;
        self.deg[v] += 1;
        self.xor[u] ^= v as u32;
        self.xor[v] ^= u as u32;
        self.weight[u] ^= c;
        self.weight[v] ^= c;
    }
}

// 木のDFSにおける逆順を求める。O(N)
// treeは破壊される
// xorは親を表す。weightは親との距離を表す。
fn dfs_rev_order(tree: &mut WeightedXorTree, root: usize) -> Vec<usize> {
    let n = tree.deg.len();
    tree.xor[root] = root as u32;
    tree.deg[root] = 0;

    let mut order = Vec::with_capacity(n - 1);
    for i in 0..n {
        let mut v = i;
        while tree.deg[v] == 1 {
            order.push(v);
            tree.deg[v] = 0;
            let p = tree.xor[v] as usize;
            if tree.deg[p] == 0 {
                break;
            }
            tree.deg[p] -= 1;
            tree.xor[p] ^= v as u32;
            tree.weight[p] ^= tree.weight[v];
            v = p;
        }
    }
    order
}

// 木の直径を求める。O(N)
// 戻り値は (直径の長さ, 直径を構成する頂点の列)
// treeは破壊される
fn tree_diameter(tree: &mut WeightedXorTree) -> (usize, Vec<usize>) {
    let mut ctree = tree.clone();
    let order = dfs_rev_order(&mut ctree, 0);
    let parent = ctree.xor;
    let weight = ctree.weight;
    let mut dist = vec![0; tree.deg.len()];
    for &v in order.iter().rev() {
        dist[v] = dist[parent[v] as usize] + weight[v];
    }
    let (farthest, _) = dist.iter().enumerate().max_by_key(|&(_, d)| d).unwrap();

    let order = dfs_rev_order(tree, farthest);
    let parent = &tree.xor;
    let weight = &tree.weight;
    let mut dist = vec![0; tree.deg.len()];
    for &v in order.iter().rev() {
        dist[v] = dist[parent[v] as usize] + weight[v];
    }
    let (other_end, _) = dist.iter().enumerate().max_by_key(|&(_, d)| d).unwrap();

    let mut path = Vec::with_capacity(tree.deg.len());
    let mut sum = 0;
    let mut v = other_end;
    while v != farthest {
        path.push(v);
        sum += weight[v];
        v = parent[v] as usize;
    }
    path.push(farthest);

    (sum, path)
}
