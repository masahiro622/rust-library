fn superset_zeta_transform(a: &mut [Mint]) {
    let n = a.len();
    for i in 0..n.trailing_zeros() {
        for j in 0..n {
            if (j & (1 << i)) == 0 {
                a[j] += a[j ^ (1 << i)];
            }
        }
    }
}

fn superset_mobius_transform(a: &mut [Mint]) {
    let n = a.len();
    for i in 0..n.trailing_zeros() {
        for j in 0..n {
            if (j & (1 << i)) == 0 {
                a[j] -= a[j ^ (1 << i)];
            }
        }
    }
}

fn superset_convolution(a: &mut Vec<Mint>, b: &mut [Mint]) -> Vec<Mint> {
    let n = a.len();
    superset_zeta_transform(a);
    superset_zeta_transform(b);
    for i in 0..n {
        a[i] *= b[i];
    }
    superset_mobius_transform(a);
    std::mem::take(a)
}
