/// 与えられたスライスを辞書順で前の順列に変換する。
/// 前の順列が存在すれば true、最初の順列だった場合は false を返す。
pub fn prev_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();
    if n < 2 {
        return false;
    }

    // 1. a[i - 1] > a[i] となる最大の i を探す
    let mut i = n - 1;
    while i > 0 && a[i - 1] <= a[i] {
        i -= 1;
    }
    if i == 0 {
        a.reverse();
        return false; // 最初の順列だった
    }

    // 2. a[j] < a[i - 1] となる最大の j を探す
    let mut j = n - 1;
    while a[j] >= a[i - 1] {
        j -= 1;
    }

    // 3. a[i - 1] と a[j] を交換
    a.swap(i - 1, j);

    // 4. a[i..] を反転（降順にする）
    a[i..].reverse();

    true
}
