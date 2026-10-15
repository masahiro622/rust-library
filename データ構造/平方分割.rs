struct SqrtDecomposition {
    n: usize,          // 元の配列のサイズ
    block_size: usize, // 1ブロックあたりのサイズ (√N)
    data: Vec<i64>,    // 生のデータ
    buckets: Vec<i64>, // ブロックごとの事前計算値（ここでは和）
}

impl SqrtDecomposition {
    // 初期化
    fn new(data: Vec<i64>) -> Self {
        let n = data.len();
        // ブロックサイズは √N (1の場合は1にする)
        let block_size = (n as f64).sqrt().ceil() as usize;
        let block_count = (n + block_size - 1) / block_size;

        let mut buckets = vec![0; block_count];

        // 各ブロックの和を計算
        for (i, &val) in data.iter().enumerate() {
            buckets[i / block_size] += val;
        }

        SqrtDecomposition {
            n,
            block_size,
            data,
            buckets,
        }
    }

    // 1点加算: indexの位置にxを加える
    // 計算量: O(1)
    fn add(&mut self, index: usize, x: i64) {
        self.data[index] += x;
        // 対応するバケットも更新
        let b_idx = index / self.block_size;
        self.buckets[b_idx] += x;
    }

    // 区間取得: [l, r) の和を求める
    // 計算量: O(√N)
    fn query(&self, mut l: usize, r: usize) -> i64 {
        let mut sum = 0;

        // 1. 左側の端数処理
        // ブロックの先頭になるまで、またはrに到達するまで1つずつ足す
        while l < r && l % self.block_size != 0 {
            sum += self.data[l];
            l += 1;
        }

        // 2. ブロック単位の処理
        // ブロックごと飛ばせる間はバケットの値を使う
        while l + self.block_size <= r {
            let b_idx = l / self.block_size;
            sum += self.buckets[b_idx];
            l += self.block_size;
        }

        // 3. 右側の端数処理
        // 残りを1つずつ足す
        while l < r {
            sum += self.data[l];
            l += 1;
        }

        sum
    }
}
