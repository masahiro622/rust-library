use ac_library::Monoid;

pub struct M;

impl Monoid for M {
    type S = i64;

    fn identity() -> Self::S {
        0 // 単位元：gcd(x, 0) = x
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        gcd(*a, *b)
    }
}

// 補助関数
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}
