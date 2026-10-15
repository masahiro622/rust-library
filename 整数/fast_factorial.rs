pub fn fast_factorial_precalculate() -> Vec<Mint> {
    let k = 9;
    let b = 1usize << k;
    let mut f = vec![vec![]; k + 1];
    f[0] = vec![Mint::raw(1)];
    for i in 0..k {
        let pow2_i = 1usize << i;
        let shift_count = 3 * pow2_i;
        let shifted = if f[i].len() >= pow2_i {
            shift_of_sampling_points(&f[i], pow2_i, shift_count)
        } else {
            vec![Mint::raw(0); shift_count]
        };
        let mut extended = f[i].clone();
        extended.extend(shifted);
        f[i + 1] = Vec::with_capacity(1usize << (i + 1));
        for j in 0..(1usize << (i + 1)) {
            let val1 = extended[2 * j];
            let val2 = extended[2 * j + 1];
            f[i + 1].push(val1 * val2 * Mint::new(pow2_i) * Mint::new(2 * j + 1));
        }
    }
    let max_index = Mint::modulus() as usize / b;
    let add_need = if max_index > b { max_index - b + 1 } else { 0 };
    let mut final_fk = f[k].clone();
    if add_need > 0 && !f[k].is_empty() {
        final_fk.extend(shift_of_sampling_points(&f[k], b, add_need));
    }
    let mut result = vec![Mint::raw(0); max_index + 1];
    result[0] = Mint::raw(1);
    for i in 1..=max_index {
        result[i] = final_fk[i - 1] * result[i - 1] * Mint::new(b) * Mint::new(i);
    }
    result
}

pub fn fast_factorial(n: usize, ctx: &[Mint]) -> Mint {
    let factorials = ctx;
    let k = 9;
    let b = 1usize << k;
    let block = n / b;
    let rem = n % b;
    let mut ans = factorials[block];
    let base = block * b;
    for i in 1..=rem {
        ans *= Mint::new(base + i);
    }
    ans
}
