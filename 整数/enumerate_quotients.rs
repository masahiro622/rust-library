fn generate_floor_sequence(n: usize) -> Vec<usize> {
    let mut k = isqrt(n);
    if k * k + k <= n {
        k += 1;
    }
    let m = n / k;
    let mut ans = vec![];
    for i in 1..m + 1 {
        ans.push(i);
    }
    for i in (1..k).rev() {
        ans.push(n / i);
    }
    ans
}
