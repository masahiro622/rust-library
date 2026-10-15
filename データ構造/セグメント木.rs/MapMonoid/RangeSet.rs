use ac_library::MapMonoid;

struct SetMapMonoid;

impl MapMonoid for SetMapMonoid {
    type M = M; // 上で定義した Monoid
    type F = i64; // 更新操作の型（ここでは "値をセットする"）

    fn identity_map() -> Self::F {
        i64::MIN // 無効な値を表す
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if *f == i64::MIN {
            *x // 更新なし
        } else {
            *f // f を代入
        }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        if *f == i64::MIN {
            *g
        } else {
            *f
        }
    }
}
