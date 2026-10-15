pub fn bernoulli_numbers(n: usize) -> FPS {
    let (fact, inv_fact) = FPS::factorials(n + 2);
    let mut f = FPS(inv_fact[1..=n + 1].to_vec()).inv();
    for i in (2..=n).step_by(2) {
        f[i] *= fact[i];
    }
    f
}
