use std::collections::HashMap;
use std::io::{self, BufRead};

const MOD: u64 = 998244353;

fn modinv(a: u64, m: u64) -> u64 {
    let (mut a, mut b, mut u, mut v) = (a as i64, m as i64, 1, 0);
    while b != 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    (u.rem_euclid(m as i64)) as u64
}

fn phi_sum(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&v) = memo.get(&n) {
        return v;
    }

    let mut res = n % MOD;
    res = res * ((n + 1) % MOD) % MOD;
    res = res * modinv(2, MOD) % MOD;

    let mut i = 2;
    while i <= n {
        let div = n / i;
        let next = n / div + 1;
        let count = (next - i) as u64;

        let sub = phi_sum(div, memo) * count % MOD;
        res = (res + MOD - sub) % MOD;
        i = next;
    }

    memo.insert(n, res);
    res
}
