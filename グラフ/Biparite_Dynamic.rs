pub struct BipartiteUnionFind {
    parent: Vec<usize>,
    weight: Vec<u32>,
    /// 根と同じパリティを持つ頂点の数 (根でのみ正しい値を持つ)
    pub count_same: Vec<usize>,
    /// 根と異なるパリティを持つ頂点の数 (根でのみ正しい値を持つ)
    pub count_diff: Vec<usize>,
    pub is_bipartite: bool,
}

impl BipartiteUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            weight: vec![0; n],
            count_same: vec![1; n], // 初期状態は自分が根で、自身のみ
            count_diff: vec![0; n], // 異なるパリティは0
            is_bipartite: true,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.parent[x];
            let root = self.find(p);
            self.weight[x] ^= self.weight[p];
            self.parent[x] = root;
            root
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);

        if rx == ry {
            if self.weight[x] == self.weight[y] {
                self.is_bipartite = false;
            }
            return;
        }

        // rx から ry への重み (パリティ) を計算
        let w = self.weight[x] ^ self.weight[y] ^ 1;

        self.parent[rx] = ry;
        self.weight[rx] = w;

        // パリティの違いに応じてサイズをマージ
        if w == 0 {
            self.count_same[ry] += self.count_same[rx];
            self.count_diff[ry] += self.count_diff[rx];
        } else {
            // パリティが反転するため、交差させて足し込む
            self.count_same[ry] += self.count_diff[rx];
            self.count_diff[ry] += self.count_same[rx];
        }
    }

    /// 頂点 x が属する連結成分の (根と同じパリティの数, 根と異なるパリティの数) を取得
    pub fn get_counts(&mut self, x: usize) -> (usize, usize) {
        let root = self.find(x);
        (self.count_same[root], self.count_diff[root])
    }
}
