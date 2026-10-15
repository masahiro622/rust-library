use ac_library::Monoid;

pub struct M;

impl Monoid for M {
    type S = i64;

    fn identity() -> Self::S {
        i64::MAX // 単位元（最大値）
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        std::cmp::min(*a, *b)
    }
}
