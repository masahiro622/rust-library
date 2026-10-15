pub struct AddOp;
impl Monoid for AddOp {
    type T = i64;
    fn identity() -> Self::T {
        0
    }
    fn binary_operate(a: &Self::T, b: &Self::T) -> Self::T {
        a + b
    }
}

pub trait Monoid {
    type T: Clone;
    fn identity() -> Self::T;
    fn binary_operate(a: &Self::T, b: &Self::T) -> Self::T;
}

pub struct DisjointSparseTable<Op: Monoid> {
    table: Vec<Vec<Op::T>>,
}

impl<Op: Monoid> DisjointSparseTable<Op> {
    /// O(N log N) で構築
    pub fn new(arr: &[Op::T]) -> Self {
        let n = arr.len();
        assert!(n > 0, "Array must not be empty");

        let mut table = Vec::new();
        // n == 1 の場合は max_k = 0 とする
        let max_k = if n == 1 { 0 } else { (n - 1).ilog2() as usize };

        for k in 0..=max_k {
            let mut row = arr.to_vec();
            let half_size = 1 << k;
            let block_size = 1 << (k + 1);

            // ブロックごとに、中点から左右への累積を計算する
            for block_start in (0..n).step_by(block_size) {
                let mid = block_start + half_size;
                if mid >= n {
                    break;
                } // 右半分が存在しない場合はスキップ

                // 左半分: mid - 1 から左へ累積 (row[mid - 1] は既に arr[mid - 1])
                for i in (block_start..mid - 1).rev() {
                    row[i] = Op::binary_operate(&arr[i], &row[i + 1]);
                }

                // 右半分: mid から右へ累積 (row[mid] は既に arr[mid])
                let end = (block_start + block_size).min(n);
                for i in mid + 1..end {
                    row[i] = Op::binary_operate(&row[i - 1], &arr[i]);
                }
            }
            table.push(row);
        }

        Self { table }
    }

    /// 半開区間 [l, r) の結果を O(1) で取得
    pub fn query(&self, l: usize, r: usize) -> Op::T {
        let n = self.table[0].len();
        assert!(l <= r && r <= n, "Invalid query range");

        // 空区間
        if l == r {
            return Op::identity();
        }

        // 半開区間 [l, r) を閉区間 [l, r - 1] に変換
        let right = r - 1;

        // 1要素の場合は計算不要
        if l == right {
            return self.table[0][l].clone();
        }

        // l と right の最上位の異なるビット位置 k を見つける
        // この k の階層では、l と right の間に必ずブロックの中点(mid)が存在する
        let k = (l ^ right).ilog2() as usize;

        // row[l] は l から mid-1 までの累積
        // row[right] は mid から right までの累積
        // 非可換な演算（行列積など）にも対応できるよう、必ず l 側の結果を左に置く
        Op::binary_operate(&self.table[k][l], &self.table[k][right])
    }
}
