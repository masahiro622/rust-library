use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct FastMap<V> {
    keys: Vec<u64>,
    vals: Vec<V>,
    used: Vec<bool>,
    mask: usize,
    fixed_random: u64, // C++のstatic変数の代わりにメンバとして保持
}

impl<V> FastMap<V>
where
    V: Default + Clone, // 値は初期化(Default)と複製(Clone)が可能である必要がある
{
    pub fn new(n: usize) -> Self {
        let mut s = 1;
        while s < n * 2 {
            s *= 2;
        }

        // 時間ベースの乱数シード (C++のchrono相当)
        let fixed_random = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        Self {
            keys: vec![0; s],
            vals: vec![V::default(); s],
            used: vec![false; s],
            mask: s - 1,
            fixed_random,
        }
    }

    fn hash(&self, mut x: u64) -> usize {
        x = x.wrapping_add(self.fixed_random);
        // Rustはデバッグビルドでオーバーフローするとパニックするため wrapping_* を使う
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
        ((x ^ (x >> 31)) as usize) & self.mask
    }

    fn index(&self, k: u64) -> usize {
        let mut i = self.hash(k);
        while self.used[i] && self.keys[i] != k {
            i = (i + 1) & self.mask;
        }
        i
    }

    // C++の operator[] 相当
    // Rustでは [] での挿入は一般的ではないため、get_mut または access という名前にするのが通例
    pub fn get_mut(&mut self, k: u64) -> &mut V {
        let i = self.index(k);
        if !self.used[i] {
            self.used[i] = true;
            self.keys[i] = k;
            self.vals[i] = V::default();
        }
        &mut self.vals[i]
    }

    // C++の get 相当
    pub fn get(&self, k: u64, default_value: V) -> V {
        let i = self.index(k);
        if self.used[i] {
            self.vals[i].clone()
        } else {
            default_value
        }
    }
}

// 使用例
fn main() {
    let mut mp = FastMap::<i64>::new(10);

    // 書き込み (operator[] の代わり)
    *mp.get_mut(100) = 5;
    *mp.get_mut(200) = 10;

    // 加算などの操作
    *mp.get_mut(100) += 3;

    // 読み込み
    println!("100: {}", mp.get(100, -1)); // -> 8
    println!("200: {}", mp.get(200, -1)); // -> 10
    println!("999: {}", mp.get(999, -1)); // -> -1 (存在しない場合)
}
