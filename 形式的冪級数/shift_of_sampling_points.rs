pub fn shift_of_sampling_points(f_values: &[Mint], c: usize, m: usize) -> Vec<Mint> {
    let n = f_values.len();
    if n == 0 || m == 0 {
        return vec![];
    }

    let (fact, fact_inv) = FPS::factorials(n.max(m));

    let f_div_fact: Vec<Mint> = f_values
        .iter()
        .enumerate()
        .map(|(j, &fj)| fj * fact_inv[j])
        .collect();

    let sign_div_fact: Vec<Mint> = (0..n).map(|k| FPS::sign(k) * fact_inv[k]).collect();

    let mut coeffs = convolution(&f_div_fact, &sign_div_fact);
    coeffs.truncate(n);

    let a_prime: Vec<Mint> = (0..n).rev().map(|idx| coeffs[idx] * fact[idx]).collect();

    let c_mint = Mint::new(c);
    let mut c_pow = Mint::raw(1);
    let mut c_pow_div_fact = Vec::with_capacity(n);
    for j in 0..n {
        c_pow_div_fact.push(c_pow * fact_inv[j]);
        c_pow *= c_mint - Mint::new(j);
    }

    let conv_b = convolution(&a_prime, &c_pow_div_fact);

    let b_div_fact: Vec<Mint> = (0..n)
        .map(|j| {
            let b_val = conv_b.get(n - 1 - j).copied().unwrap_or(Mint::raw(0));
            b_val * fact_inv[j]
        })
        .collect();

    let final_conv = convolution(&b_div_fact, &fact_inv[..m]);

    final_conv
        .into_iter()
        .take(m)
        .enumerate()
        .map(|(k, val)| val * fact[k])
        .collect()
}
