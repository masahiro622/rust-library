// Suffix Array (O(n log n)
// lcp_array (O(n)) 隣り合うsuffixのLCPを求める
// count_distinct_substrings (O(n)) 連続部分文字列の個数を求める
// longest_common_substring (O(n log n)) 2つの文字列の最長共通部分文字列を求める
fn suffix_array(s: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank = s.iter().map(|&c| c as usize).collect::<Vec<_>>();
    let mut tmp = vec![0; n];
    let mut k = 1;

    let max_rank = 256.max(n); // 最大ランク

    while k < n {
        // (rank[i], rank[i + k]) を基にソート
        // Counting Sort 2回でソート（後ろ→前）

        // ソート2回目：rank[i + k]
        {
            let mut cnt = vec![0; max_rank + 1];
            for &i in &sa {
                let r = if i + k < n { rank[i + k] } else { 0 };
                cnt[r + 1] += 1;
            }
            for i in 1..=max_rank {
                cnt[i] += cnt[i - 1];
            }
            let mut sa2 = vec![0; n];
            for &i in sa.iter().rev() {
                let r = if i + k < n { rank[i + k] } else { 0 };
                cnt[r + 1] -= 1;
                sa2[cnt[r + 1]] = i;
            }
            sa = sa2;
        }

        // ソート1回目：rank[i]
        {
            let mut cnt = vec![0; max_rank + 1];
            for &i in &sa {
                cnt[rank[i] + 1] += 1;
            }
            for i in 1..=max_rank {
                cnt[i] += cnt[i - 1];
            }
            let mut sa2 = vec![0; n];
            for &i in sa.iter().rev() {
                cnt[rank[i] + 1] -= 1;
                sa2[cnt[rank[i] + 1]] = i;
            }
            sa = sa2;
        }

        // ランク更新
        tmp[sa[0]] = 0;
        for i in 1..n {
            let a = sa[i - 1];
            let b = sa[i];
            let same = rank[a] == rank[b]
                && (if a + k < n { rank[a + k] } else { 0 })
                    == (if b + k < n { rank[b + k] } else { 0 });
            tmp[b] = if same { tmp[a] } else { tmp[a] + 1 };
        }
        std::mem::swap(&mut rank, &mut tmp);
        if rank[sa[n - 1]] == n - 1 {
            break;
        }
        k <<= 1;
    }

    sa
}
fn lcp_array(text: &str, sa: &[usize]) -> Vec<usize> {
    let n = text.len();
    let text = text.as_bytes();
    let mut rank = vec![0; n];
    for (i, &s) in sa.iter().enumerate() {
        rank[s] = i;
    }

    let mut lcp = vec![0; n];
    let mut h = 0;
    for i in 0..n {
        if rank[i] == 0 {
            h = 0;
            continue;
        }
        let j = sa[rank[i] - 1];
        while i + h < n && j + h < n && text[i + h] == text[j + h] {
            h += 1;
        }
        lcp[rank[i]] = h;
        h = h.saturating_sub(1);
    }
    lcp
}
fn count_distinct_substrings(s: &str) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    let sa = suffix_array(s);
    let lcp = lcp_array(s, &sa);
    n * (n + 1) / 2 - lcp.iter().sum::<usize>()
}
fn longest_common_substring(a: &str, b: &str) -> (usize, usize, usize) {
    let sep = '#';
    assert!(!a.contains(sep) && !b.contains(sep)); // セパレータの衝突防止

    let combined = format!("{}{}{}", a, sep, b);
    //println!("ok");
    let sa = suffix_array(&combined);

    let lcp = lcp_array(&combined, &sa);

    let n = a.len();
    let mut max_len = 0;
    let mut a_pos = 0;
    let mut b_pos = 0;

    for i in 1..sa.len() {
        let (i1, i2) = (sa[i - 1], sa[i]);

        let from_a_1 = i1 < n;
        let from_b_2 = i2 > n;
        let from_a_2 = i2 < n;
        let from_b_1 = i1 > n;

        if ((from_a_1 && from_b_2) || (from_a_2 && from_b_1)) && lcp[i] > max_len {
            max_len = lcp[i];
            if from_a_1 && from_b_2 {
                a_pos = i1;
                b_pos = i2 - n - 1;
            } else {
                a_pos = i2;
                b_pos = i1 - n - 1;
            }
        }
    }

    (max_len, a_pos, b_pos)
}
