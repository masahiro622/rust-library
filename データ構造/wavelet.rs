pub struct BitVector {
    size: usize,
    array: Vec<usize>,
    sum: Vec<u32>,
}

impl BitVector {
    const WIDTH: usize = std::mem::size_of::<usize>() * 8;
    pub fn new(array: &[bool]) -> Self {
        const W: usize = BitVector::WIDTH;
        let size = array.len();
        let mut buf = vec![0usize; size / W + 1];
        for (buf, a) in buf.iter_mut().zip(array.chunks(W)) {
            for (i, a) in a.iter().enumerate() {
                *buf |= (*a as usize) << i;
            }
        }
        let mut sum = vec![0; size / W + 1];
        let mut acc = 0;
        for (sum, buf) in sum[1..].iter_mut().zip(buf.iter()) {
            acc += buf.count_ones();
            *sum = acc;
        }
        BitVector {
            size,
            array: buf,
            sum,
        }
    }
    fn quot_rem(n: usize) -> (usize, usize) {
        const S: usize = BitVector::WIDTH.trailing_zeros() as usize;
        (n >> S, n & ((1 << S) - 1))
    }
    pub fn rank(&self, k: usize) -> usize {
        assert!(k <= self.size);
        let (q, r) = Self::quot_rem(k);
        unsafe {
            (*self.sum.get_unchecked(q)
                + (self.array.get_unchecked(q) & ((1 << r) - 1)).count_ones()) as usize
        }
    }
    pub fn select(&self, _k: usize) -> Option<usize> {
        unimplemented!();
    }
    pub fn at(&self, k: usize) -> bool {
        assert!(k < self.size);
        let (q, r) = Self::quot_rem(k);
        self.array[q] >> r & 1 == 1
    }
}

pub struct WaveletMatrix {
    size: usize,
    mat: Vec<BitVector>,
    zeros: Vec<usize>,
}

impl WaveletMatrix {
    pub fn new(array: &[u32], bits: usize) -> Self {
        assert!(bits == 32 || array.iter().all(|p| *p < (1 << bits)));
        let size = array.len();
        let mut array = array.to_vec();
        let mut buf = vec![false; size];
        let mut tmp = vec![Vec::with_capacity(size); 2];
        let mut mat = Vec::with_capacity(bits);
        let mut zeros = Vec::with_capacity(bits);
        for i in (0..bits).rev() {
            for (buf, a) in buf.iter_mut().zip(array.iter()) {
                let x = ((*a >> i) & 1) == 1;
                *buf = x;
                tmp[x as usize].push(*a);
            }
            mat.push(BitVector::new(&buf));
            zeros.push(tmp[0].len());
            array.clear();
            array.extend(tmp[0].drain(..));
            array.extend(tmp[1].drain(..));
        }
        mat.reverse();
        zeros.reverse();
        WaveletMatrix {
            size: size,
            mat: mat,
            zeros: zeros,
        }
    }

    // 元の配列のk番目の要素を取得
    pub fn access(&self, mut k: usize) -> u32 {
        assert!(k < self.size);
        let mut ans = 0;
        for (i, (bits, zero)) in self.mat.iter().zip(self.zeros.iter()).enumerate().rev() {
            if bits.at(k) {
                // 1の場合
                ans |= 1 << i;
                k = *zero + bits.rank(k);
            } else {
                // 0の場合
                k = k - bits.rank(k);
            }
        }
        ans
    }

    // [l..r) からk番目に小さいものを返す
    pub fn kth_smallest(&self, mut l: usize, mut r: usize, mut k: usize) -> usize {
        assert!(r <= self.size);
        assert!(l < r);
        assert!(k < r - l);
        let mut ans = 0;
        for (i, (bits, zero)) in self.mat.iter().zip(self.zeros.iter()).enumerate().rev() {
            let l_cnt = bits.rank(l);
            let r_cnt = bits.rank(r);
            let z_cnt = (r - l) - (r_cnt - l_cnt);
            if k >= z_cnt {
                l = *zero + l_cnt;
                r = *zero + r_cnt;
                ans |= 1 << i as u64;
                k -= z_cnt;
            } else {
                l = l - l_cnt;
                r = r - r_cnt;
            }
        }
        ans
    }

    // [l..r) からk番目に大きいものを返す
    pub fn kth_largest(&self, l: usize, r: usize, k: usize) -> usize {
        assert!(r <= self.size);
        assert!(l < r);
        assert!(k < r - l);
        // k番目に大きい = (r-l-1-k)番目に小さい
        self.kth_smallest(l, r, (r - l - 1) - k)
    }

    // [l..r) の範囲でupper未満の要素の個数を返す
    pub fn range_freq(&self, mut l: usize, mut r: usize, upper: u32) -> usize {
        assert!(r <= self.size);
        if l >= r {
            return 0;
        }

        let mut count = 0;
        for (i, (bits, zero)) in self.mat.iter().zip(self.zeros.iter()).enumerate().rev() {
            let bit = ((upper >> i) & 1) == 1;
            let l_cnt = bits.rank(l);
            let r_cnt = bits.rank(r);

            if bit {
                // upperのi番目のビットが1の場合
                // 0のグループの要素は全て条件を満たす
                count += (r - l) - (r_cnt - l_cnt);
                // 1のグループで探索を続ける
                l = *zero + l_cnt;
                r = *zero + r_cnt;
            } else {
                // upperのi番目のビットが0の場合
                // 0のグループでのみ探索を続ける
                l = l - l_cnt;
                r = r - r_cnt;
            }
        }
        count
    }

    // [l..r) の範囲でupper未満の最大値を返す（upper未満の最後の値）
    pub fn prev_value(&self, l: usize, r: usize, upper: u32) -> Option<u32> {
        let count = self.range_freq(l, r, upper);
        if count == 0 {
            None
        } else {
            Some(self.kth_smallest(l, r, count - 1) as u32)
        }
    }

    // [l..r) の範囲でlower以上の最小値を返す（lower以上の最初の値）
    pub fn next_value(&self, l: usize, r: usize, lower: u32) -> Option<u32> {
        let total_count = r - l;
        let count = self.range_freq(l, r, lower);
        if count == total_count {
            None
        } else {
            Some(self.kth_smallest(l, r, count) as u32)
        }
    }

    // [l..r) の範囲での値xの出現回数を返す
    pub fn count(&self, l: usize, r: usize, x: u32) -> usize {
        assert!(r <= self.size);
        if l >= r {
            return 0;
        }

        // x未満の要素数 と (x+1)未満の要素数 の差分を計算
        let count_less = self.range_freq(l, r, x);
        let count_less_or_equal = self.range_freq(l, r, x + 1);
        count_less_or_equal - count_less
    }

    // [l..r) の範囲で [min_val, max_val) の値を持つ要素の個数を返す
    pub fn range_count(&self, l: usize, r: usize, min_val: u32, max_val: u32) -> usize {
        if min_val >= max_val {
            return 0;
        }
        self.range_freq(l, r, max_val) - self.range_freq(l, r, min_val)
    }

    // 配列の長さを返す
    pub fn len(&self) -> usize {
        self.size
    }

    // 配列が空かどうかを返す
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // [l..r) の範囲で値xが最初に現れる位置を返す
    pub fn first_occurrence(&self, l: usize, r: usize, x: u32) -> Option<usize> {
        if self.count(l, r, x) == 0 {
            None
        } else {
            // 二分探索で最初の出現位置を探す
            let mut left = l;
            let mut right = r;
            while right - left > 1 {
                let mid = (left + right) / 2;
                if self.count(l, mid, x) > 0 {
                    right = mid;
                } else {
                    left = mid;
                }
            }
            Some(left)
        }
    }

    // [l..r) の範囲で値xが最後に現れる位置を返す
    pub fn last_occurrence(&self, l: usize, r: usize, x: u32) -> Option<usize> {
        if self.count(l, r, x) == 0 {
            None
        } else {
            // 二分探索で最後の出現位置を探す
            let mut left = l;
            let mut right = r;
            while right - left > 1 {
                let mid = (left + right) / 2;
                if self.count(mid, r, x) > 0 {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            Some(left)
        }
    }
}
