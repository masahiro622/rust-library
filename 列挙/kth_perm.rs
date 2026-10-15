// 順列(0..n-1)の辞書順順位を求める
// O(n log n)

fn permutation_rank(p: &[usize]) -> usize {
    let n = p.len();

    // 階乗を前計算
    let mut fact = vec![1i64; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as i64;
    }

    // Fenwick木を1..nに初期化
    let mut bit = ac_library::fenwicktree::FenwickTree::new(n, 0i64);
    for i in 0..n {
        bit.add(i, 1);
    }
    // 順位計算
    let mut rank = 0;
    for i in 0..n {
        let smaller = bit.sum(0..p[i]); // p[i]より小さい数の個数
        rank += smaller * fact[n - 1 - i];
        bit.add(p[i], -1); // p[i]を使った
    }
    rank as usize
}

struct Fenwick {
    n: usize,
    bit: Vec<usize>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            n,
            bit: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut i: usize, x: isize) {
        while i <= self.n {
            self.bit[i] = (self.bit[i] as isize + x) as usize;
            i += i & i.wrapping_neg();
        }
    }

    fn sum(&self, mut i: usize) -> usize {
        let mut s = 0;
        while i > 0 {
            s += self.bit[i];
            i &= i - 1;
        }
        s
    }

    /// k番目の1を持つ位置（1-indexed）を探す: sum(pos) >= k となる最小のpos
    fn find_kth(&self, mut k: usize) -> usize {
        let mut pos = 0;
        let mut bit_mask = 1 << (usize::BITS - 1 - self.n.leading_zeros()); // 最大2^m <= n
        while bit_mask > 0 {
            let next = pos + bit_mask as usize;
            if next <= self.n && self.bit[next] < k {
                k -= self.bit[next];
                pos = next;
            }
            bit_mask >>= 1;
        }
        pos + 1
    }
}

fn kth_permutation(n: usize, mut k: usize) -> Vec<usize> {
    // 1. 階乗を前計算
    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i;
    }

    // 2. Fenwick木を初期化（1..nを未使用とする）
    let mut bit = Fenwick::new(n);
    for i in 1..=n {
        bit.add(i, 1);
    }

    // 3. 左から順に要素を決める
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let block_size = fact[n - 1 - i];
        let idx = k / block_size;
        k %= block_size;
        let x = bit.find_kth(idx + 1); // idxは0-indexed
        result.push(x);
        bit.add(x, -1); // 使用済みにする
    }

    result
}
