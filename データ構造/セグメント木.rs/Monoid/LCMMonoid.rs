use ac_library::Monoid;

pub struct M;

impl Monoid for M {
    type S = i64;

    fn identity() -> Self::S {
        1 // 単位元（lcm(x, 1) = x）
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        lcm(*a, *b)
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

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}
