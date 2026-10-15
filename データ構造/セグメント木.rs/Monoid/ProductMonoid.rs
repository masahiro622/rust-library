use ac_library::Monoid;

pub struct M;

impl Monoid for M {
    type S = i64;

    fn identity() -> Self::S {
        1 // 単位元（1）
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a * *b
    }
}
