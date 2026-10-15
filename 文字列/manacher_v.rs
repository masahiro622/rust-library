// O(n)
pub fn manacher<T: Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut a = vec![0; 2 * n + 1];
    let mut i = 1;
    let mut j = 1;
    while i <= 2 * n {
        // 1. 伸ばせるだけ伸ばす
        while j < i && i + j < 2 * n && s[(i - j) / 2 - 1] == s[(i + j) / 2] {
            j += 2;
        }
        a[i] = j;
        // ※ 空区間になった場合は例外的な対応が必要
        if j == 0 {
            i += 1;
            j = 1;
            continue;
        }
        // 2. 境界に達しない限り、回文配列を書き写す
        let mut k = 1;
        while k <= i && k + a[i - k] < j {
            a[i + k] = a[i - k];
            k += 1;
        }
        // 3. 境界に達したら現在の回文区間を覚えて 1 に戻る
        i += k;
        j -= k;
    }
    a.remove(0);
    a.remove(a.len() - 1);
    a
}
