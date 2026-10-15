use rand::Rng;
#[cfg(test)]
mod tests {
    use super::*; // 上で定義した solve や brute を読み込む

    #[test]
    fn random_test() {
        let mut rng = rand::thread_rng();
        for _ in 0..20_000 {
            // 例: a の生成
            let n = rng.gen_range(1..=30);
            let k = rng.gen_range(1..=10);
            let mut a = vec![];
            for _ in 0..n {
                a.push(rng.gen_range(1..=100));
            }

            let fast = solve(n, k, a);
            let slow = brute(n, k, a);

            assert_eq!(
                fast, slow,
                "WA!\nn: {}\nk: {}\na: {:?}\nFast: {}\nSlow: {}",
                n, k, a, fast, slow
            );
        }
    }
}
