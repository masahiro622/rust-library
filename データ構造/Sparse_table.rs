pub struct MinOp;
impl IdempotentOp for MinOp {
    type T = i32;
    fn operate(a: &Self::T, b: &Self::T) -> Self::T {
        *a.min(b)
    }
}

/// 冪等性（A op A = A）を持つ演算を定義するトレイト
pub trait IdempotentOp {
    type T: Clone;
    fn operate(a: &Self::T, b: &Self::T) -> Self::T;
}

/// 抽象化Sparse Table
pub struct SparseTable<Op: IdempotentOp> {
    table: Vec<Vec<Op::T>>,
}

impl<Op: IdempotentOp> SparseTable<Op> {
    /// O(N log N) で構築
    pub fn new(arr: &[Op::T]) -> Self {
        let n = arr.len();
        assert!(n > 0, "Array must not be empty");

        let mut table = vec![arr.to_vec()];
        let mut k = 1;

        // (1 << k) が n 以下の間、次の段を構築する
        while (1 << k) <= n {
            let row_len = n - (1 << k) + 1;
            let mut row = Vec::with_capacity(row_len);

            let step = 1 << (k - 1);
            for i in 0..row_len {
                let res = Op::operate(&table[k - 1][i], &table[k - 1][i + step]);
                row.push(res);
            }
            table.push(row);
            k += 1;
        }

        Self { table }
    }

    /// 半開区間 [l, r) の結果を O(1) で取得
    pub fn query(&self, l: usize, r: usize) -> Op::T {
        assert!(l < r && r <= self.table[0].len(), "Invalid query range");

        let len = r - l;
        let k = len.ilog2() as usize;

        Op::operate(&self.table[k][l], &self.table[k][r - (1 << k)])
    }
}
