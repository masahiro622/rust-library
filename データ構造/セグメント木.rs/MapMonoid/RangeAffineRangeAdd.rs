struct RangeSum;
impl Monoid for RangeSum {
    type S = (Mint, usize); // (sum, size) の2要素にする
    fn identity() -> Self::S {
        (Mint::raw(0), 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let (a_sum, a_size) = *a;
        let (b_sum, b_size) = *b;
        (a_sum + b_sum, a_size + b_size)
    }
}

struct CompositeRangeAdd;
impl MapMonoid for CompositeRangeAdd {
    type M = RangeSum;

    type F = (Mint, Mint); // f(x) = ax + b
    fn identity_map() -> Self::F {
        (Mint::raw(1), Mint::raw(0)) // f(x) = 1*x + 0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        let (a, b) = *f;
        let (sum, size) = *x;
        // 全要素に a を掛け、定数 b は区間の要素数(size)の分だけ足す
        (a * sum + b * Mint::new(size), size)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        let (a, b) = *f;
        let (c, d) = *g;
        (a * c, a * d + b) // f(g(x)) = a(cx + d) + b = acx + ad + b
    }
}
