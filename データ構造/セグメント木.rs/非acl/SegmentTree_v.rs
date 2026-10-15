pub struct SegmentTree {
    size: usize,
    data: Vec<i64>,
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        SegmentTree {
            size,
            //dataの初期値は区間最大値(最小値)ならi64::MIN(MAX)
            data: vec![0; 2 * size],
        }
    }

    /// 一点更新：index 番目に value を加算
    pub fn update(&mut self, mut index: usize, value: i64) {
        index += self.size;
        self.data[index] += value; //一点代入なら+=を=に変更
        while index > 1 {
            index /= 2;
            self.data[index] = self.data[2 * index] + self.data[2 * index + 1]; //区間最大値(最小値)なら右辺+ -> max(min)
        }
    }

    /// 区間和 [l, r) を求める
    pub fn query(&self, mut l: usize, mut r: usize) -> i64 {
        l += self.size;
        r += self.size;
        let mut sum = 0; //区間最大値(最小値)ならsum = i64::MIN(MAX)
        while l < r {
            if l % 2 == 1 {
                sum += self.data[l]; //区間最大値（最小値）ならsum = sum.max(self.data[l])(minに変更);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                sum += self.data[r]; //区間最大値（最小値）ならsum = sum.max(self.data[r])(minに変更);
            }
            l /= 2;
            r /= 2;
        }
        sum
    }

    /// index番目の現在の値を取得する
    pub fn get(&self, index: usize) -> i64 {
        self.data[index + self.size]
    }
}
