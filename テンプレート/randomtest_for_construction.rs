struct XorShift64 {
    x: u64,
}
impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self { x: seed }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.x;
        x ^= x << 7;
        x ^= x >> 9;
        x ^= x << 8;
        self.x = x;
        x
    }
    fn gen_usize(&mut self, lo: usize, hi: usize) -> usize {
        // [lo, hi]
        let w = hi - lo + 1;
        lo + (self.next_u64() as usize % w)
    }
    fn gen_i64(&mut self, lo: i64, hi: i64) -> i64 {
        // [lo, hi]
        let w = (hi - lo + 1) as u64;
        lo + (self.next_u64() % w) as i64
    }
}

fn check() -> bool {}

fn main() {
    let mut rng = XorShift64::new(123456789);

    for _ in 0..20_000 {
        let n = rng.gen_usize(1, 30);

        let answer = solve(n, k, &a);
        if !check(answer) {
            println! {"{}",answer};
            return;
        }
    }
}
