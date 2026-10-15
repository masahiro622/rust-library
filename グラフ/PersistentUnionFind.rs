#[derive(Debug, Clone)]
pub struct PersistentUnionFind {
    parent: Vec<Vec<(usize, usize)>>, // (version, parent)
    size: Vec<Vec<(usize, usize)>>,   // (version, size)
    now: usize,                       // 現在のバージョン番号
}

impl PersistentUnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![vec![]; n];
        let mut size = vec![vec![]; n];
        for i in 0..n {
            parent[i].push((0, i));
            size[i].push((0, 1));
        }
        Self {
            parent,
            size,
            now: 0,
        }
    }

    // バージョン v における x の親を取得
    pub fn find(&self, x: usize, v: usize) -> usize {
        let mut cur = x;
        loop {
            let p = self.get_latest(&self.parent[cur], v);
            if p == cur {
                return cur;
            }
            cur = p;
        }
    }

    // バージョン v において x, y を結合。新しいバージョンを返す
    pub fn unite(&mut self, x: usize, y: usize, v: usize) -> usize {
        let mut x_root = self.find(x, v);
        let mut y_root = self.find(y, v);

        if x_root == y_root {
            self.now += 1;
            return self.now;
        }

        let x_size = self.get_latest(&self.size[x_root], v);
        let y_size = self.get_latest(&self.size[y_root], v);

        // union by size
        if x_size < y_size {
            std::mem::swap(&mut x_root, &mut y_root);
        }

        let new_version = self.now + 1;
        self.parent[y_root].push((new_version, x_root));
        self.size[x_root].push((new_version, x_size + y_size));

        self.now = new_version;
        new_version
    }

    // バージョン v において x, y が同じ集合か？
    pub fn same(&self, x: usize, y: usize, v: usize) -> bool {
        self.find(x, v) == self.find(y, v)
    }

    // バージョン v における x の属する集合のサイズ
    pub fn size(&self, x: usize, v: usize) -> usize {
        let root = self.find(x, v);
        self.get_latest(&self.size[root], v)
    }

    fn get_latest(&self, history: &Vec<(usize, usize)>, v: usize) -> usize {
        let mut ok = 0;
        let mut ng = history.len();
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if history[mid].0 <= v {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        history[ok].1
    }
}
