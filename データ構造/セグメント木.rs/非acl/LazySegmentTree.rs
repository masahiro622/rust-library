pub struct LazySegmentTree {
    size: usize,
    data: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        LazySegmentTree {
            size,

            data: vec![0; 2 * size], //dataの初期値は区間最大値(最小値)ならi64::MIN(MAX)
            lazy: vec![0; 2 * size], //区間更新が代入ならlazy: Vec<Option<i64>>,
        }
    }

    fn push(&mut self, node: usize, l: usize, r: usize) {
        if self.lazy[node] != 0 {
            //最大値、最小値取得なら(r-l)を掛けない
            self.data[node] += self.lazy[node] * (r - l) as i64;

            if node < self.size {
                self.lazy[node * 2] += self.lazy[node];
                self.lazy[node * 2 + 1] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }
    }
    /*//区間更新が代入のとき
        fn push(&mut self, node: usize, l: usize, r: usize) {
        if let Some(val) = self.lazy[node] {
            self.data[node] = val;
            if node < self.size {
                self.lazy[node * 2] = Some(val);
                self.lazy[node * 2 + 1] = Some(val);
            }
            self.lazy[node] = None;
        }
    } */
    pub fn range_add(&mut self, a: usize, b: usize, x: i64) {
        self._range_add(a, b, x, 1, 0, self.size);
    }

    fn _range_add(&mut self, a: usize, b: usize, x: i64, node: usize, l: usize, r: usize) {
        self.push(node, l, r);
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            self.lazy[node] += x; //区間更新が代入なら右辺Some(x);
            self.push(node, l, r);
        } else {
            let mid = (l + r) / 2;
            self._range_add(a, b, x, node * 2, l, mid);
            self._range_add(a, b, x, node * 2 + 1, mid, r);

            self.data[node] = self.data[node * 2] + self.data[node * 2 + 1]; //区間最大値(最小値)なら右辺+ -> max(min)
        }
    }

    pub fn range_sum(&mut self, a: usize, b: usize) -> i64 {
        self._range_sum(a, b, 1, 0, self.size)
    }

    fn _range_sum(&mut self, a: usize, b: usize, node: usize, l: usize, r: usize) -> i64 {
        self.push(node, l, r);
        if r <= a || b <= l {
            //dataの初期値にする
            0
        } else if a <= l && r <= b {
            self.data[node]
        } else {
            let mid = (l + r) / 2;
            self._range_sum(a, b, node * 2, l, mid) + self._range_sum(a, b, node * 2 + 1, mid, r)
        }
    }
}
