/// 簡易乱数生成器（線形合同法）
struct SimpleRng {
    state: u64,
}
impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next(&mut self) -> u64 {
        // 64bit LCGパラメータ
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }
    /// [low, high) の範囲で乱数を返す
    fn gen_range(&mut self, low: usize, high: usize) -> usize {
        low + (self.next() as usize % (high - low))
    }
}

// 例: pollard_rho内で使う
fn pollard_rho(n: usize) -> usize {
    if n % 2 == 0 {
        return 2;
    }
    // シードは適当に現在時刻などから
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut rng = SimpleRng::new(seed);

    loop {
        let c = rng.gen_range(1, n);
        let mut x = rng.gen_range(2, n);
        let mut y = x;
        let mut d = 1usize;
        let f = |x: usize| (mod_mul(x, x, n) + c) % n;

        while d == 1 {
            x = f(x);
            y = f(f(y));
            d = gcd((x as i64 - y as i64).abs() as usize, n);
        }

        if d != n {
            return d;
        }
    }
}
