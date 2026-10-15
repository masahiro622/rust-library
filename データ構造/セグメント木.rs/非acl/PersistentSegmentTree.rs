#[derive(Clone)]
pub struct PersistentSegmentTree {
    n: usize,
    data: Vec<i32>,
    left: Vec<Option<usize>>,
    right: Vec<Option<usize>>,
    root: Vec<usize>,
    size: usize,
}

impl PersistentSegmentTree {
    pub fn new(n: usize) -> Self {
        let mut tree = PersistentSegmentTree {
            n,
            data: vec![0; 4 * n], // segment tree array for data
            left: vec![None; 4 * n],
            right: vec![None; 4 * n],
            root: vec![0; n + 1], // to store roots for each version
            size: 0,
        };
        tree
    }

    fn build(&mut self, v: usize, tl: usize, tr: usize, arr: &Vec<i32>) {
        if tl == tr {
            self.data[v] = arr[tl];
        } else {
            let tm = (tl + tr) / 2;
            let left_child = self.size + 1;
            let right_child = left_child + 1;

            self.build(left_child, tl, tm, arr);
            self.build(right_child, tm + 1, tr, arr);

            self.data[v] = self.data[left_child] + self.data[right_child];
        }
    }

    fn query_range(&self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> i32 {
        if r < tl || tr < l {
            return 0;
        }
        if l <= tl && tr <= r {
            return self.data[v];
        }
        let tm = (tl + tr) / 2;
        let left_child = v * 2 + 1;
        let right_child = left_child + 1;

        let left_sum = self.query_range(left_child, tl, tm, l, r);
        let right_sum = self.query_range(right_child, tm + 1, tr, l, r);

        left_sum + right_sum
    }

    pub fn update(
        &mut self,
        v: usize,     // 更新元となるバージョンのルートノード
        tl: usize,    // 更新元の範囲の左端
        tr: usize,    // 更新元の範囲の右端
        pos: usize,   // 更新する位置
        new_val: i32, // 新しい値
    ) -> usize // 新しいバージョンのルートノードを返す
    {
        if tl == tr {
            let new_idx = self.size;
            self.size += 1;

            let left_child = self.left[v].unwrap();
            let right_child = self.right[v].unwrap();

            let mut left = self.data.clone();
            let mut right = self.data.clone();

            left[pos] = new_val;
            right[pos] = self.size;
            self.root.push(v);
        }
    }
}

/// 使用例
fn main() {
    // 配列を使って永続セグメント木を構築
    let arr = vec![1, 2, 3, 4, 5];
    let mut tree = PersistentSegmentTree::new(arr.len());

    // 配列を使って木をビルド
    tree.build(0, 0, arr.len() - 1, &arr);

    // 最初のバージョンで範囲[1, 3]の合計を求める
    let sum1 = tree.query_range(0, 0, arr.len() - 1, 1, 3);
    println!("最初のバージョンでの範囲[1, 3]の合計: {}", sum1);

    // 配列の1番目の要素を更新
    tree.update(0, 0, arr.len() - 1, 1, 10);

    // 更新後の新しいバージョンで範囲[1, 3]の合計を求める
    let sum2 = tree.query_range(1, 0, arr.len() - 1, 1, 3);
    println!("更新後のバージョンでの範囲[1, 3]の合計: {}", sum2);

    // 最初のバージョンで範囲[1, 3]の合計を再確認（永続的に保持されている）
    let sum1_again = tree.query_range(0, 0, arr.len() - 1, 1, 3);
    println!("最初のバージョンでの範囲[1, 3]の合計: {}", sum1_again);
}
