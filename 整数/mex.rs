//ある集合に含まれない最小の非負整数を求める
use std::collections::HashSet;

pub fn mex(values: &[usize]) -> usize {
    let mut seen = HashSet::new();
    for &v in values {
        seen.insert(v);
    }

    for i in 0.. {
        if !seen.contains(&i) {
            return i;
        }
    }
    unreachable!()
}
//値が大きすぎない(<10^6)場合は、これを使った方が速い
pub fn fast_mex(values: &[usize]) -> usize {
    let n = values.len();
    let mut present = vec![false; n + 2]; // 最大でも n+1 まで調べればOK

    for &v in values {
        if v < present.len() {
            present[v] = true;
        }
    }

    for i in 0..present.len() {
        if !present[i] {
            return i;
        }
    }
    unreachable!()
}
