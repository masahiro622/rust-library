pub fn stirling_numbers_second_row(n: usize) -> FPS {
    if n == 0 {
        return FPS(vec![Mint::raw(1)]);
    }
    let (_, inv) = FPS::factorials(n + 1);
    let mut a = vec![Mint::raw(0); n + 1];
    let mut b = vec![Mint::raw(0); n + 1];

    for i in 0..=n {
        a[i] = inv[i];
        if i % 2 == 1 {
            a[i] = -a[i];
        }
        b[i] = Mint::new(i).pow(n as u64) * inv[i];
    }

    let conv = convolution(&a, &b);
    FPS(conv[0..=n].to_vec())
}

pub fn stirling_numbers_second_column(n_max: usize, k: usize) -> FPS {
    if k > n_max {
        return FPS(vec![]);
    }
    if k == 0 {
        let mut res = vec![Mint::raw(0); n_max + 1];
        res[0] = Mint::raw(1);
        return FPS(res);
    }

    let limit_deg = n_max - k;
    let (fact, inv) = FPS::factorials(n_max + limit_deg + 1);

    let mut q_poly = vec![Mint::raw(0); limit_deg + 1];
    q_poly[..(limit_deg + 1)].copy_from_slice(&inv[1..(limit_deg + 2)]);

    let r_poly = FPS(q_poly).pow(k as i64);

    let mut result = Vec::with_capacity(limit_deg + 1);
    let inv_fact_k = inv[k];

    for m in 0..=limit_deg {
        let val = fact[k + m] * inv_fact_k * r_poly[m];
        result.push(val);
    }

    FPS(result)
}
