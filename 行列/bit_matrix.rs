/// 2値の文字列行列 A[N][M] と B[M][K] を受け取り、
/// C[N][K] = A * B mod 2 を文字列形式で返す。
/// 各行は "0101..." のような文字列で、C[N][K] の Vec<String>。
fn bit_matrix_mul_mod2(a_strs: &[String], b_strs: &[String]) -> Vec<String> {
    let n = a_strs.len();
    let m = a_strs[0].len();
    let k = b_strs[0].len();

    // 行列A: 各行をu128として保持
    let mut a_bits = vec![0u128; n];
    for (i, row_str) in a_strs.iter().enumerate() {
        let mut row = 0u128;
        for (j, c) in row_str.chars().enumerate() {
            if c == '1' {
                row |= 1 << j;
            }
        }
        a_bits[i] = row;
    }

    // 行列B: 各列をu128として保持
    let mut b_bits = vec![0u128; k];
    for (i, row_str) in b_strs.iter().enumerate() {
        for (j, c) in row_str.chars().enumerate() {
            if c == '1' {
                b_bits[j] |= 1 << i;
            }
        }
    }

    // 結果C: Vec<String>で返す
    let mut result = vec![String::new(); n];
    for i in 0..n {
        for j in 0..k {
            let x = a_bits[i] & b_bits[j];
            result[i].push(char::from_digit(x.count_ones() % 2, 10).unwrap());
        }
    }
    result
}
