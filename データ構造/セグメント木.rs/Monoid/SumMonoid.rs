use ac_library::Monoid;

pub struct SumMonoid;

impl Monoid for SumMonoid {
    type S = i64;

    fn identity() -> Self::S {
        0 // 単位元
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a + *b
    }
}
