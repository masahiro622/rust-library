// n の整数分割数を求める
// O(n^(3/2))

const MOD: i64 = 998244353;
pub fn partition_numbers(n: usize) -> Vec<i64> {
    let mut p = vec![0i64; n + 1];
    p[0] = 1;

    for i in 1..=n {
        let mut k = 1;
        while {
            let pent1 = k * (3 * k - 1) / 2;
            let pent2 = k * (3 * k + 1) / 2;
            if pent1 <= i {
                let sign = if k % 2 == 1 { 1 } else { MOD - 1 };
                p[i] = (p[i] + sign * p[i - pent1] % MOD) % MOD;
            }
            if pent2 <= i {
                let sign = if k % 2 == 1 { 1 } else { MOD - 1 };
                p[i] = (p[i] + sign * p[i - pent2] % MOD) % MOD;
            }
            pent1 <= i || pent2 <= i
        } {
            k += 1;
        }
    }
    p
}
