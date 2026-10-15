// RangeAdd,RangeSum

pub struct SumLen;
impl Monoid for SumLen {
    type S = (i64, i64); // (和, 区間長)
    fn identity() -> Self::S {
        (0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
}

pub struct RangeAdd;
impl MapMonoid for RangeAdd {
    type M = SumLen;
    type F = i64;
    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &(i64, i64)) -> (i64, i64) {
        (x.0 + f * x.1, x.1)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}
