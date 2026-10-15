#[allow(clippy::many_single_char_names)]
pub fn convolution_mod_1000000007(a: &[u32], b: &[u32]) -> Vec<u32> {
    const MOD: u64 = 1_000_000_007;
    const M1: u64 = 754_974_721;
    const M2: u64 = 167_772_161;
    const M3: u64 = 469_762_049;
    const M1M2: u64 = M1 * M2;

    modulus!(M1, M2, M3);

    if a.is_empty() || b.is_empty() {
        return vec![];
    }

    let c1 = convolution_raw::<u32, M1>(a, b);
    let c2 = convolution_raw::<u32, M2>(a, b);
    let c3 = convolution_raw::<u32, M3>(a, b);

    let (_, inv_m1_mod_m2) = inv_gcd(M1 as i64, M2 as i64);
    let (_, inv_m1m2_mod_m3) = inv_gcd((M1M2 % M3) as i64, M3 as i64);
    let m1_mod = M1 % MOD;
    let m1m2_mod = M1M2 % MOD;

    c1.into_iter()
        .zip(c2)
        .zip(c3)
        .map(|((x1, x2), x3)| {
            let x1 = x1 as u64;

            let t1 = ((x2 as i64 - x1 as i64).rem_euclid(M2 as i64) * inv_m1_mod_m2)
                .rem_euclid(M2 as i64) as u64;

            let x12_mod_m3 = ((x1 as u128 + M1 as u128 * t1 as u128) % M3 as u128) as i64;
            let t2 = ((x3 as i64 - x12_mod_m3).rem_euclid(M3 as i64) * inv_m1m2_mod_m3)
                .rem_euclid(M3 as i64) as u64;

            ((x1 % MOD + m1_mod * (t1 % MOD) + m1m2_mod * (t2 % MOD)) % MOD) as u32
        })
        .collect()
}
