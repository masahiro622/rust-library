/// パターン文字列 `pattern` に対して、部分一致テーブルを構築する関数
/// `pi[i]` は `pattern[..=i]` の proper prefix と suffix の最大一致長
pub fn prefix_function(pattern: &str) -> Vec<usize> {
    let pat: Vec<char> = pattern.chars().collect();
    let mut pi = vec![0; pat.len()];
    let mut j = 0;

    for i in 1..pat.len() {
        while j > 0 && pat[i] != pat[j] {
            j = pi[j - 1];
        }
        if pat[i] == pat[j] {
            j += 1;
        }
        pi[i] = j;
    }

    pi
}
/// テキスト `text` 中にパターン `pattern` が出現するすべての開始位置を返す
pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let txt: Vec<char> = text.chars().collect();
    let pat: Vec<char> = pattern.chars().collect();
    let pi = prefix_function(pattern);
    let mut result = vec![];

    let mut j = 0;
    for i in 0..txt.len() {
        while j > 0 && txt[i] != pat[j] {
            j = pi[j - 1];
        }
        if txt[i] == pat[j] {
            j += 1;
        }
        if j == pat.len() {
            result.push(i + 1 - j);
            j = pi[j - 1];
        }
    }

    result
}
