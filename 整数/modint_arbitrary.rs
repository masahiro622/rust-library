use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ModInt {
    value: i64,
    modulo: i64,
}

impl ModInt {
    fn new(value: i64, modulo: i64) -> Self {
        let mut v = value % modulo;
        if v < 0 {
            v += modulo;
        }
        Self { value: v, modulo }
    }

    fn pow(self, mut exp: i64) -> Self {
        let mut base = self;
        let mut result = ModInt::new(1, self.modulo);
        while exp > 0 {
            if exp % 2 == 1 {
                result *= base;
            }
            base *= base;
            exp /= 2;
        }
        result
    }

    fn inv(self) -> Self {
        // フェルマーの小定理 (mod が素数のときのみ)
        assert!(self.modulo > 1);
        self.pow(self.modulo - 2)
    }
}

// 加算
impl Add for ModInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        assert_eq!(self.modulo, rhs.modulo);
        ModInt::new(self.value + rhs.value, self.modulo)
    }
}
impl AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// 減算
impl Sub for ModInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        assert_eq!(self.modulo, rhs.modulo);
        ModInt::new(self.value - rhs.value, self.modulo)
    }
}
impl SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// 乗算
impl Mul for ModInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        assert_eq!(self.modulo, rhs.modulo);
        ModInt::new(
            (self.value as i128 * rhs.value as i128 % self.modulo as i128) as i64,
            self.modulo,
        )
    }
}
impl MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// 除算
impl Div for ModInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert_eq!(self.modulo, rhs.modulo);
        self * rhs.inv()
    }
}
impl DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

// let comb = Comb::new(100000, modulo);
//println!("5! = {:?}", comb.factorial(5));
//のようにして使う

// ---- 前計算をまとめる構造体 ----
struct Comb {
    fact: Vec<ModInt>,
    inv_fact: Vec<ModInt>,
    modulo: i64,
}

impl Comb {
    fn new(size: usize, modulo: i64) -> Self {
        let mut fact = vec![ModInt::new(1, modulo); size + 1];
        let mut inv_fact = vec![ModInt::new(1, modulo); size + 1];

        for i in 1..=size {
            fact[i] = fact[i - 1] * ModInt::new(i as i64, modulo);
        }
        inv_fact[size] = fact[size].inv();
        for i in (1..=size).rev() {
            inv_fact[i - 1] = inv_fact[i] * ModInt::new(i as i64, modulo);
        }

        Self {
            fact,
            inv_fact,
            modulo,
        }
    }

    fn factorial(&self, n: usize) -> ModInt {
        self.fact[n]
    }

    fn permutation(&self, n: usize, r: usize) -> ModInt {
        if r > n {
            return ModInt::new(0, self.modulo);
        }
        self.fact[n] * self.inv_fact[r] * self.inv_fact[n - r]
    }

    fn combination(&self, n: usize, r: usize) -> ModInt {
        if r > n {
            return ModInt::new(0, self.modulo);
        }
        self.fact[n] * self.inv_fact[n - r]
    }

    fn homogeneous(&self, n: usize, r: usize) -> ModInt {
        // nHr = (n+r-1)Cr
        if n == 0 && r > 0 {
            return ModInt::new(0, self.modulo);
        }
        self.combination(n + r - 1, r)
    }

    fn catalan(&self, n: usize) -> ModInt {
        if n == 0 {
            return ModInt::new(1, self.modulo);
        }
        let two_n_choose_n = self.combination(2 * n, n);
        two_n_choose_n / ModInt::new((n + 1) as i64, self.modulo)
    }
}
