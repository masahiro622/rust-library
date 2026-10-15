/// 高速 floor sum
///  sum_{0 <= i < n} floor((a * i + b) / m) O(log(a))
pub fn floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64 {
    let (div_a, rem_a) = (a.div_euclid(m), a.rem_euclid(m));
    let (div_b, rem_b) = (b.div_euclid(m), b.rem_euclid(m));

    let mut res = n * (n - 1) / 2 * div_a + n * div_b;
    if rem_a == 0 {
        return res;
    }

    let k = (rem_a * n - rem_a + rem_b).div_euclid(m);
    res += n * k;
    res += floor_sum(k, rem_a, m, rem_b - m * k);

    res
}
//min{a *i + b mod m | 0 <= i < n} O((log(m)^2))
pub fn min_of_mod_of_linear(n: i64, m: i64, a: i64, b: i64) -> i64 {
    let b = b % m;
    let x = floor_sum(n, m, a, b);
    if floor_sum(n, m, a, b + m - 1) < x + n {
        return 0;
    }
    let mut ok = b;
    let mut ng = 0;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if floor_sum(n, m, a, b + m - mid - 1) < x + n {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
