// 依存: データ構造/Mo.rs (Mo)

/// 木上の Mo 法（パスクエリ）
/// DFS で入るときと出るときの両方を記録したオイラーツアー（長さ 2N）上の区間に変換して Mo に渡す。
/// 区間内にちょうど 1 回現れる頂点がパス上の頂点になるので、伸縮は頂点のトグルで行う。
pub struct TreeMo {
    tin: Vec<usize>,
    tout: Vec<usize>,
    euler: Vec<usize>,   // euler[t]: 時刻 t に入った / 出た頂点
    up: Vec<Vec<usize>>, // up[k][v]: v の 2^k 個上の祖先（根より上は根）
    mo: Mo,
    // クエリごとに solve の間だけ追加する頂点（区間に含まれない LCA）
    lcas: Vec<Option<usize>>,
}

impl TreeMo {
    /// n: 頂点数, edges: 辺のリスト (長さ n-1), root: 根の頂点番号
    pub fn new(n: usize, edges: Vec<(usize, usize)>, root: usize) -> Self {
        // XOR linked tree: 葉から順に剥がして親と部分木サイズを求める
        let mut deg = vec![0u32; n];
        let mut parent = vec![0; n];
        for (u, v) in edges {
            deg[u] += 1;
            deg[v] += 1;
            parent[u] ^= v;
            parent[v] ^= u;
        }
        parent[root] = root;
        deg[root] = 0;

        let mut size = vec![1; n];
        let mut mx_child_sz = vec![0; n];
        let mut order = Vec::with_capacity(n); // 剥がした順（子が親より先）
        for i in 0..n {
            let mut v = i;
            while deg[v] == 1 {
                deg[v] = 0;
                order.push(v);
                let p = parent[v];
                size[p] += size[v];
                mx_child_sz[p] = mx_child_sz[p].max(size[v]);
                if deg[p] == 0 {
                    break;
                }
                deg[p] -= 1;
                parent[p] ^= v;
                v = p;
            }
        }

        // 根から順に、各子の部分木にオイラーツアー上の連続区間 [tin, tin + 2 * size) を割り当てる。
        // 部分木最大の子を最後に回すと、パスが長い木で Mo の移動量が 1 割ほど減る（実測）
        let mut tin = vec![0; n];
        let mut tout = vec![2 * n - 1; n];
        let mut offset = vec![1; n]; // 次の子に割り当てる tin の、親の tin からのオフセット
        for &v in order.iter().rev() {
            let p = parent[v];
            if mx_child_sz[p] == size[v] {
                mx_child_sz[p] = 0;
                tin[v] = tin[p] + 2 * (size[p] - size[v]) - 1;
            } else {
                tin[v] = tin[p] + offset[p];
                offset[p] += 2 * size[v];
            }
            tout[v] = tin[v] + 2 * size[v] - 1;
        }
        let mut euler = vec![0; 2 * n];
        for v in 0..n {
            euler[tin[v]] = v;
            euler[tout[v]] = v;
        }

        let log = (usize::BITS - n.leading_zeros()) as usize;
        let mut up = vec![parent];
        for k in 1..log {
            let next: Vec<usize> = (0..n).map(|v| up[k - 1][up[k - 1][v]]).collect();
            up.push(next);
        }

        TreeMo {
            tin,
            tout,
            euler,
            up,
            mo: Mo::new(),
            lcas: vec![],
        }
    }

    /// a が b の祖先（a == b を含む）か
    fn is_ancestor(&self, a: usize, b: usize) -> bool {
        self.tin[a] <= self.tin[b] && self.tout[b] <= self.tout[a]
    }

    /// 最小共通祖先 (LCA)。O(log N)
    pub fn lca(&self, u: usize, v: usize) -> usize {
        if self.is_ancestor(u, v) {
            return u;
        }
        if self.is_ancestor(v, u) {
            return v;
        }
        // v の祖先にならない範囲で u を上げきると、その親が LCA
        let mut u = u;
        for k in (0..self.up.len()).rev() {
            if !self.is_ancestor(self.up[k][u], v) {
                u = self.up[k][u];
            }
        }
        self.up[0][u]
    }

    /// 頂点 v の親（根の親は根自身）
    pub fn parent(&self, v: usize) -> usize {
        self.up[0][v]
    }

    /// 頂点 u から v へのパス上の頂点（両端と LCA を含む）を対象とするクエリを追加
    pub fn insert(&mut self, u: usize, v: usize) {
        let (u, v) = if self.tin[u] <= self.tin[v] { (u, v) } else { (v, u) };
        if self.is_ancestor(u, v) {
            self.mo.insert(self.tin[u], self.tin[v] + 1);
            self.lcas.push(None);
        } else {
            self.mo.insert(self.tout[u], self.tin[v] + 1);
            self.lcas.push(Some(self.lca(u, v)));
        }
    }

    /// 頂点 u から v へのパス上の辺を対象とするクエリを追加
    /// 辺 (parent(c), c) は子側の頂点 c として add / del に渡される（パス上の頂点から LCA を除いたもの）
    pub fn insert_edge(&mut self, u: usize, v: usize) {
        let (u, v) = if self.tin[u] <= self.tin[v] { (u, v) } else { (v, u) };
        let l = if self.is_ancestor(u, v) { self.tin[u] + 1 } else { self.tout[u] };
        self.mo.insert(l, self.tin[v] + 1);
        self.lcas.push(None);
    }

    /// 木上の Mo 法を実行する
    /// add(state, v) / del(state, v): 頂点 v をパスに追加 / パスから削除する
    /// solve(state, idx): 第2引数はクエリの元のインデックス（insert / insert_edge を呼んだ順）
    /// 可変参照を共有するものはすべてstateにまとめて渡すこと。
    /// add, del は let で定義せずクロージャーを直接渡すこと。
    pub fn execute<S, Add, Del, Solve>(&self, state: &mut S, add: Add, del: Del, mut solve: Solve)
    where
        Add: FnMut(&mut S, usize),
        Del: FnMut(&mut S, usize),
        Solve: FnMut(&mut S, usize),
    {
        let mut inner = TreeMoState {
            state,
            in_path: vec![false; self.tin.len()],
            add,
            del,
        };
        let euler = &self.euler;
        let lcas = &self.lcas;

        // 区間の伸縮はどの向きでも「その位置の頂点をトグル」になる
        self.mo.execute(
            &mut inner,
            |s, i| s.toggle(euler[i]),
            |s, i| s.toggle(euler[i]),
            |s, i| s.toggle(euler[i]),
            |s, i| s.toggle(euler[i]),
            |s, idx| match lcas[idx] {
                Some(c) => {
                    (s.add)(s.state, c);
                    solve(s.state, idx);
                    (s.del)(s.state, c);
                }
                None => solve(s.state, idx),
            },
        );
    }
}

/// execute 内で Mo に渡す状態。add / del は Mo の 5 つのクロージャすべてから呼ぶのでここにまとめる
struct TreeMoState<'a, S, Add, Del> {
    state: &'a mut S,
    in_path: Vec<bool>, // 現在の区間に奇数回現れる（= パス上にある）か
    add: Add,
    del: Del,
}

impl<S, Add, Del> TreeMoState<'_, S, Add, Del>
where
    Add: FnMut(&mut S, usize),
    Del: FnMut(&mut S, usize),
{
    fn toggle(&mut self, v: usize) {
        self.in_path[v] = !self.in_path[v];
        if self.in_path[v] {
            (self.add)(self.state, v);
        } else {
            (self.del)(self.state, v);
        }
    }
}
