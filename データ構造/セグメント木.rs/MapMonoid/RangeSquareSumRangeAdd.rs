pub struct M;
impl Monoid for M {
    type S = (Mint, Mint, Mint); // 区間の長さ、区間の値の和、区間の値の2乗の和

    fn identity() -> Self::S {
        (Mint::raw(0), Mint::raw(0), Mint::raw(0))
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1, a.2 + b.2)
    }
}

pub struct RangeAdd;

impl MapMonoid for RangeAdd {
    type M = M; // 値の型（例えば区間和）
    type F = Mint;
    fn identity_map() -> Self::F {
        Mint::raw(0)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f + *g
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        let (sum, sum_a, sum_a2) = *x;
        let a = *f;
        (
            sum,
            sum_a + sum * a,
            sum_a2 + Mint::raw(2) * sum_a * a + sum * a * a,
        )
    }
}
