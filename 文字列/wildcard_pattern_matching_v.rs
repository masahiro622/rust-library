use std::io;

const MOD: u32 = 998244353;
const PRIMITIVE_ROOT: u32 = 3;

fn mod_pow(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r: u64 = 1;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = (r * a) % m;
        }
        a = (a * a) % m;
        e >>= 1;
    }
    r
}

fn mod_inv(x: u64) -> u64 {
    mod_pow(x, (MOD as u64) - 2, MOD as u64)
}

fn bit_reverse(a: &mut [u32]) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
}

fn ntt(a: &mut [u32], invert: bool) {
    let n = a.len();
    bit_reverse(a);
    let mut len = 2usize;
    while len <= n {
        // wlen = primitive_root^{(MOD-1)/len}
        let wlen = mod_pow(
            PRIMITIVE_ROOT as u64,
            ((MOD - 1) / (len as u32)) as u64,
            MOD as u64,
        ) as u32;
        let wlen = if invert {
            mod_inv(wlen as u64) as u32
        } else {
            wlen
        };
        let mut i = 0;
        while i < n {
            let mut w = 1u32;
            for j in 0..(len / 2) {
                let u = a[i + j] as u64;
                let v = (a[i + j + len / 2] as u64 * w as u64) % (MOD as u64);
                let x = (u + v) % (MOD as u64);
                let y = (u + MOD as u64 - v) % (MOD as u64);
                a[i + j] = x as u32;
                a[i + j + len / 2] = y as u32;
                w = ((w as u64 * wlen as u64) % (MOD as u64)) as u32;
            }
            i += len;
        }
        len <<= 1;
    }
    if invert {
        let inv_n = mod_inv(n as u64) as u32;
        for x in a.iter_mut() {
            *x = ((*x as u64 * inv_n as u64) % (MOD as u64)) as u32;
        }
    }
}

fn convolution(a: &[u32], b: &[u32]) -> Vec<u32> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut n = 1usize;
    while n < a.len() + b.len() - 1 {
        n <<= 1;
    }
    let mut fa = vec![0u32; n];
    let mut fb = vec![0u32; n];
    fa[..a.len()].copy_from_slice(a);
    fb[..b.len()].copy_from_slice(b);
    ntt(&mut fa, false);
    ntt(&mut fb, false);
    for i in 0..n {
        fa[i] = ((fa[i] as u64 * fb[i] as u64) % (MOD as u64)) as u32;
    }
    ntt(&mut fa, true);
    fa.truncate(a.len() + b.len() - 1);
    fa
}

// シンプルなsplitmix64乱数生成器
struct SplitMix64 {
    state: u64,
}
impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        let mut z = {
            self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
            self.state
        };
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() & 0xFFFF_FFFF) as u32
    }
}

fn is_wild(c: char) -> bool {
    c == '?' || c == '*'
}

fn wildcard_pattern_matching(s: &str, t: &str) -> Vec<bool> {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let n = s_chars.len();
    let m = t_chars.len();
    if m == 0 || n < m {
        // Tが空の場合の動作定義：ここでは何も出力しないか全てマッチするか；しかし問題ではm>=1
        return vec![];
    }

    let mut rng = SplitMix64::new(123456789); // 固定シードで決定的にする場合

    // ランダムベクター r_i (mod MOD)
    let mut r: Vec<u32> = vec![0u32; m];
    for i in 0..m {
        r[i] = rng.next_u32() % MOD;
    }

    // q = r_i * T'_i を構築 (T'_i = ワイルドカードでなければ1)
    // および q2 = q * value(T_i)
    // 値については、文字を数値にマップする；異なる記号を区別するために(byte as u32)を使用
    let mut q = vec![0u32; m];
    let mut q2 = vec![0u32; m];
    for i in 0..m {
        if !is_wild(t_chars[i]) {
            let rv = r[i] as u64;
            q[i] = (rv % (MOD as u64)) as u32;
            let val = t_chars[i] as u32; // 文字の値
            q2[i] = ((q[i] as u64 * (val as u64)) % (MOD as u64)) as u32;
        } else {
            q[i] = 0;
            q2[i] = 0;
        }
    }

    // A = S'_i * value(S_i) および B = S'_i を構築 (S'_i = ワイルドカードでなければ1)
    let mut a = vec![0u32; n];
    let mut b = vec![0u32; n];
    for i in 0..n {
        if !is_wild(s_chars[i]) {
            b[i] = 1;
            a[i] = (s_chars[i] as u32) % MOD;
        } else {
            b[i] = 0;
            a[i] = 0;
        }
    }
    // 畳み込みのためにqとq2を反転
    let mut q_rev = q.clone();
    q_rev.reverse();
    let mut q2_rev = q2.clone();
    q2_rev.reverse();
    // conv1 = convolution(A, q_rev)
    // conv2 = convolution(B, q2_rev)
    let conv1 = convolution(&a, &q_rev);
    let conv2 = convolution(&b, &q2_rev);

    // シフト k = 0..=n-m について、畳み込み配列のインデックスは k + m - 1
    let mut out = vec![];
    for k in 0..=n - m {
        let idx = k + m - 1;
        let v1 = if idx < conv1.len() { conv1[idx] } else { 0u32 };
        let v2 = if idx < conv2.len() { conv2[idx] } else { 0u32 };
        let diff = v1 + MOD - v2;
        if diff % MOD == 0 {
            out.push(true);
        } else {
            out.push(false);
        }
    }
    out
}
