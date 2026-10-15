/// ビットベクトル構造体
#[derive(Clone)]
struct BitVector {
    block: Vec<u64>,
    count: Vec<u32>,
    n: u32,
    zeros: u32,
}

impl BitVector {
    const W: u32 = 64;

    fn new() -> Self {
        Self {
            block: Vec::new(),
            count: Vec::new(),
            n: 0,
            zeros: 0,
        }
    }

    fn with_capacity(n: usize) -> Self {
        let mut bv = Self::new();
        bv.init(n);
        bv
    }

    fn init(&mut self, n: usize) {
        self.n = n as u32;
        self.zeros = self.n;
        let block_size = (n + Self::W as usize - 1) / Self::W as usize;
        self.block.resize(block_size, 0);
        // count は番兵つき（block_size + 1）
        self.count.resize(block_size + 1, 0);
    }

    fn get(&self, i: u32) -> u32 {
        let block_idx = (i / Self::W) as usize;
        let bit_idx = i % Self::W;
        ((self.block[block_idx] >> bit_idx) & 1) as u32
    }

    fn set(&mut self, i: u32) {
        let block_idx = (i / Self::W) as usize;
        let bit_idx = i % Self::W;
        self.block[block_idx] |= 1u64 << bit_idx;
    }

    fn build(&mut self) {
        // prefix: count[i] = sum(popcnt(block[0..i]))
        for i in 1..=self.block.len() {
            self.count[i] = self.count[i - 1] + self.block[i - 1].count_ones();
        }
        self.zeros = self.rank0(self.n);
    }

    fn rank0(&self, i: u32) -> u32 {
        i - self.rank1(i)
    }

    fn rank1(&self, i: u32) -> u32 {
        let block_idx = (i / Self::W) as usize;
        let bit_idx = i % Self::W;

        // ちょうどブロック境界（含む i == n）なら、番兵つき prefix を返すだけ
        if bit_idx == 0 {
            return self.count[block_idx];
        }

        let mask = (1u64 << bit_idx) - 1;
        self.count[block_idx] + (self.block[block_idx] & mask).count_ones()
    }
}

/// Binary Indexed Tree (フェンウィック木)
#[derive(Clone)]
struct Bit<T> {
    n: usize,
    data: Vec<T>,
}

impl<T> Bit<T>
where
    T: Clone
        + std::ops::AddAssign
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + Default,
{
    fn new() -> Self {
        Self {
            n: 0,
            data: Vec::new(),
        }
    }

    fn with_capacity(size: usize) -> Self {
        let mut bit = Self::new();
        bit.init(size);
        bit
    }

    fn init(&mut self, size: usize) {
        self.n = size;
        self.data.resize(self.n + 1, T::default());
    }

    fn add(&mut self, mut k: usize, x: T) {
        k += 1;
        while k <= self.n {
            self.data[k] += x.clone();
            k += k & k.wrapping_neg();
        }
    }

    fn sum(&self, mut k: usize) -> T {
        let mut ret = T::default();
        while k > 0 {
            ret += self.data[k].clone();
            k &= k - 1;
        }
        ret
    }

    fn range_sum(&self, l: usize, r: usize) -> T {
        if l >= r {
            return T::default();
        }
        if l == 0 {
            self.sum(r)
        } else {
            self.sum(r) - self.sum(l)
        }
    }
}

impl<T> Bit<T>
where
    T: Clone
        + std::ops::AddAssign
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + Default,
{
    fn range_sum_sub(&self, l: usize, r: usize) -> T {
        if l >= r {
            return T::default();
        }
        if l == 0 {
            self.sum(r)
        } else {
            self.sum(r) - self.sum(l)
        }
    }
}

/// ウェーブレット行列
pub struct WaveletMatrix<S, T> {
    n: usize,
    lg: usize,
    bv: Vec<BitVector>,
    bit: Vec<Bit<T>>,
    ps: Vec<(S, S)>,
    ys: Vec<S>,
}

impl<S, T> Default for WaveletMatrix<S, T>
where
    S: Clone + Ord + Copy,
    T: Clone
        + std::ops::AddAssign
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S, T> WaveletMatrix<S, T>
where
    S: Clone + Ord + Copy,
    T: Clone
        + std::ops::AddAssign
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + Default,
{
    pub fn new() -> Self {
        Self {
            n: 0,
            lg: 0,
            bv: Vec::new(),
            bit: Vec::new(),
            ps: Vec::new(),
            ys: Vec::new(),
        }
    }

    /// 点(x, y)を追加
    pub fn add_point(&mut self, x: S, y: S) {
        self.ps.push((x, y));
        self.ys.push(y);
    }

    /// データ構造を構築
    pub fn build(&mut self) {
        // 点をソートして重複を除去
        self.ps.sort_unstable();
        self.ps.dedup();
        self.n = self.ps.len();

        // y座標をソートして重複を除去
        self.ys.sort_unstable();
        self.ys.dedup();

        if self.n == 0 {
            return;
        }

        // 現在の配列とネクスト配列
        let mut cur: Vec<u32> = (0..self.n).map(|i| self.yid(self.ps[i].1)).collect();
        let mut nxt = vec![0u32; self.n];

        // ビット数を計算
        self.lg = if self.ys.len() <= 1 {
            1
        } else {
            (self.ys.len() - 1).next_power_of_two().trailing_zeros() as usize + 1
        };

        // ビットベクトルとBitを初期化
        self.bv = vec![BitVector::with_capacity(self.n); self.lg];
        self.bit = vec![Bit::with_capacity(self.n); self.lg];

        // 各レベルでウェーブレット分解
        for h in (0..self.lg).rev() {
            // 現在のビット位置で1の点を設定
            for i in 0..self.n {
                if (cur[i] >> h) & 1 != 0 {
                    self.bv[h].set(i as u32);
                }
            }
            self.bv[h].build();

            // 0と1で分離
            let mut it0 = 0;
            let mut it1 = self.bv[h].zeros as usize;

            for i in 0..self.n {
                if self.bv[h].get(i as u32) == 0 {
                    nxt[it0] = cur[i];
                    it0 += 1;
                } else {
                    nxt[it1] = cur[i];
                    it1 += 1;
                }
            }

            std::mem::swap(&mut cur, &mut nxt);
        }
    }

    /// x座標からIDを取得
    fn xid(&self, x: S) -> usize {
        self.ps.partition_point(|p| p.0 < x)
    }

    /// y座標からIDを取得
    fn yid(&self, y: S) -> u32 {
        match self.ys.binary_search(&y) {
            Ok(idx) => idx as u32,
            Err(idx) => idx as u32,
        }
    }

    /// 点(x, y)に値valを加算
    pub fn add(&mut self, x: S, y: S) -> Result<usize, &'static str> {
        let target = (x, y);
        let idx = match self.ps.binary_search(&target) {
            Ok(i) => i,
            Err(_) => return Err("Point not found"),
        };
        Ok(idx)
    }

    pub fn add_value(&mut self, x: S, y: S, val: T) -> Result<(), &'static str> {
        let target = (x, y);
        let mut i = match self.ps.binary_search(&target) {
            Ok(idx) => idx,
            Err(_) => return Err("Point not found"),
        };

        for h in (0..self.lg).rev() {
            let i0 = self.bv[h].rank0(i as u32) as usize;
            if self.bv[h].get(i as u32) != 0 {
                i += self.bv[h].zeros as usize - i0;
            } else {
                i = i0;
            }
            self.bit[h].add(i, val.clone());
        }
        Ok(())
    }

    /// 内部的な範囲和計算
    fn sum_internal(&self, l: usize, r: usize, upper: u32) -> T {
        let mut res = T::default();
        let mut l = l;
        let mut r = r;

        for h in (0..self.lg).rev() {
            let l0 = self.bv[h].rank0(l as u32) as usize;
            let r0 = self.bv[h].rank0(r as u32) as usize;

            if (upper >> h) & 1 != 0 {
                res += self.bit[h].range_sum_sub(l0, r0);
                l += self.bv[h].zeros as usize - l0;
                r += self.bv[h].zeros as usize - r0;
            } else {
                l = l0;
                r = r0;
            }
        }
        res
    }

    /// 矩形領域[L, R) × [D, U)内の値の合計
    pub fn sum(&self, l: S, d: S, r: S, u: S) -> T {
        let l_idx = self.xid(l);
        let r_idx = self.xid(r);
        let u_id = self.yid(u);
        let d_id = self.yid(d);

        self.sum_internal(l_idx, r_idx, u_id) - self.sum_internal(l_idx, r_idx, d_id)
    }
}
