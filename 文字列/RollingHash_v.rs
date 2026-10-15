const MOD: u64 = (1u64 << 61) - 1;

#[inline]
fn mod_add(a: u64, b: u64) -> u64 {
    let mut x = a + b;
    if x >= MOD {
        x -= MOD;
    }
    x
}

#[inline]
fn mod_sub(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + MOD - b
    }
}

// a*b mod (2^61-1)
#[inline]
fn mod_mul(a: u64, b: u64) -> u64 {
    let t = (a as u128) * (b as u128);
    // t を 61bit ごとに畳む
    let low = (t as u64) & MOD;
    let high = (t >> 61) as u64;
    let mut x = low + high;
    // もう一回だけ畳む（x が MOD を少し超える可能性）
    x = (x & MOD) + (x >> 61);
    if x >= MOD {
        x - MOD
    } else {
        x
    }
}

#[derive(Clone)]
pub struct RollingHash {
    base: u64,
    pow: Vec<u64>,  // base^i
    pref: Vec<u64>, // pref[i] = hash(s[0..i])
}

impl RollingHash {
    pub fn new(s: &[u8], base: u64) -> Self {
        let n = s.len();
        let mut pow = vec![0u64; n + 1];
        let mut pref = vec![0u64; n + 1];
        pow[0] = 1;
        pref[0] = 0;

        for i in 0..n {
            pow[i + 1] = mod_mul(pow[i], base);
            // 文字を 1..256 に寄せる（0 を避ける）
            let c = (s[i] as u64) + 1;
            pref[i + 1] = mod_add(mod_mul(pref[i], base), c);
        }

        Self { base, pow, pref }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.pref.len() - 1
    }

    /// s[l..r] のハッシュ
    #[inline]
    pub fn hash(&self, l: usize, r: usize) -> u64 {
        // pref[r] - pref[l]*base^(r-l)
        let x = mod_mul(self.pref[l], self.pow[r - l]);
        mod_sub(self.pref[r], x)
    }

    /// 最長共通接頭辞長（同じ文字列上で i, j から）
    /// O(log n). 頻繁に呼ぶなら suffix array / z / lcp 構築の方が速い場合あり。
    pub fn lcp(&self, i: usize, j: usize) -> usize {
        let n = self.len();
        let mut low = 0usize;
        let mut high = n.saturating_sub(i.max(j)) + 1; // 探索上限+1（半開区間）

        while high - low > 1 {
            let mid = (low + high) / 2;
            if self.hash(i, i + mid) == self.hash(j, j + mid) {
                low = mid;
            } else {
                high = mid;
            }
        }
        low
    }
}
