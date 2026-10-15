//正の整数列 a に対して、連続部分列のうち、和が S 以下 のものはいくつあるか？
fn count_subarrays_with_sum_leq(a: &[usize], s: usize) -> usize {
    let mut count = 0;
    let mut sum = 0;
    let mut right = 0;

    for left in 0..a.len() {
        while right < a.len() && sum + a[right] <= s {
            sum += a[right];
            right += 1;
        }

        count += right - left;

        if right == left {
            right += 1;
        } else {
            sum -= a[left];
        }
    }

    count
}

fn main() {
    input! {
    _:usize,k:usize,s:Chars}
    let runs = run_length_encode(&s);
    let m = runs.len();
    let mut que = VecDeque::new();
    let mut cnt = 0;
    let mut sum = 0;
    let mut ans = 0;

    let mut i = 0;
    while i < m {
        que.push_back(runs[i]);
        sum += runs[i].1;
        if runs[i].0 == '0' {
            cnt += 1;
        }
        i += 1;
        while cnt > k {
            let (r, c) = que.pop_front().unwrap();
            sum -= c;
            if r == '0' {
                cnt -= 1;
            }
        }
        ans = max(ans, sum);
    }
    println!("{}", ans);
}
