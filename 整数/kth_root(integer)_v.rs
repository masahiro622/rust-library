// floor(a^(1/k)) を求める
fn kth_root(a: usize, k: u32) -> usize {
    if k == 1 {
        return a;
    }
    let mut low = 0;
    let mut high = a.min(1 << 63); // x^k <= 2^64 なら x <= 2^63 で十分

    while low < high {
        let mid = (low + high + 1) / 2;
        if check_pow(mid, k, a) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    low
}

fn check_pow(mut x: usize, k: u32, limit: usize) -> bool {
    let mut res: u128 = 1;
    let limit = limit as u128;
    let mut k = k;
    let mut base = x as u128;

    while k > 0 {
        if k % 2 == 1 {
            res = res.saturating_mul(base);
            if res > limit {
                return false;
            }
        }
        if k > 1 {
            base = base.saturating_mul(base);
            if base > limit {
                return false;
            }
        }
        k /= 2;
    }
    true
}
