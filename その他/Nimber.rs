use std::collections::HashMap;

struct Nimber {
    val: usize,
}

impl Nimber {
    fn new(val: usize) -> Self {
        Self { val }
    }

    fn add(&self, other: &Nimber) -> Nimber {
        Nimber::new(self.val ^ other.val)
    }

    fn mul(&self, other: &Nimber, engine: &mut NimberMulEngine) -> Nimber {
        Nimber::new(engine.mul(self.val, other.val))
    }
}

fn ceil_pow2_bits(x: usize) -> u32 {
    let b = std::cmp::max(1, usize::BITS - x.leading_zeros());
    let mut p = 1_u32;
    while p < b {
        p <<= 1;
    }
    p
}

struct NimberMulEngine {
    memo: HashMap<(usize, usize, u32), usize>,
}

impl NimberMulEngine {
    fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }

    fn mul(&mut self, a: usize, b: usize) -> usize {
        if a == 0 || b == 0 {
            return 0;
        }
        let bits = ceil_pow2_bits(std::cmp::max(a, b));
        self.mul_fixed(a, b, bits)
    }

    fn mul_fixed(&mut self, a: usize, b: usize, bits: u32) -> usize {
        if bits == 1 {
            return (a & 1) * (b & 1);
        }

        let (x, y) = if a <= b { (a, b) } else { (b, a) };
        let key = (x, y, bits);
        if let Some(&v) = self.memo.get(&key) {
            return v;
        }

        let half = bits / 2;
        let mask = if half == usize::BITS {
            usize::MAX
        } else {
            (1_usize << half) - 1
        };

        let al = a & mask;
        let au = a >> half;
        let bl = b & mask;
        let bu = b >> half;

        let p = self.mul_fixed(al, bl, half);
        let q = self.mul_fixed(au, bu, half);
        let r = self.mul_fixed(al ^ au, bl ^ bu, half);

        let high = r ^ p;
        let low = p ^ self.mul_fixed(q, 1_usize << (half - 1), half);

        let value = low ^ (high << half);
        self.memo.insert(key, value);
        value
    }
}
