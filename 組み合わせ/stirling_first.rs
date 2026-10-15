pub fn stirling_first(n: usize) -> FPS {
    if n == 0 {
        return FPS(vec![Mint::raw(1)]);
    }
    let polys: Vec<FPS> = (0..n)
        .map(|i| FPS(vec![-Mint::new(i), Mint::raw(1)]))
        .collect();
    FPS::all_prod(&polys)
}

pub fn stirling_first_column(k: usize, n_max: usize) -> FPS {
    let mut ln1mx = vec![Mint::raw(0); n_max + 1];
    for i in 1..=n_max {
        ln1mx[i] = Mint::new(i).inv();
    }
    let f = FPS(ln1mx).pow(k as i64);
    let mut fact = vec![Mint::raw(1); n_max + 1];
    for i in 1..=n_max {
        fact[i] = fact[i - 1] * Mint::new(i);
    }
    let inv_k_fact = fact[k].inv();
    let mut res = vec![Mint::raw(0); n_max - k + 1];
    for n in k..=n_max {
        let coeff = f[n] * inv_k_fact * fact[n];
        res[n - k] = if (n - k) % 2 == 0 { coeff } else { -coeff };
    }
    FPS(res)
}
