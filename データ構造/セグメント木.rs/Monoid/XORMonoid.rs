use ac_library::Monoid;

pub struct M;

impl Monoid for M {
    type S = i64; // 必要に応じて u32 や usize にしてもOK

    fn identity() -> Self::S {
        0 // XORの単位元は0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a ^ *b
    }
}
