// グラフの彩色数を求めるプログラム　(O(N^2 * 2^N))
// 入力は隣接行列形式で、頂点数と辺数を最初の行に指定
// その後に各辺の頂点番号を指定する
// 出力はグラフの彩色数

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    if n == 0 {
        println!("0");
        return;
    }

    // 隣接行列をビットマスクで表現
    let mut adj = vec![0u32; n];

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let v: usize = parts.next().unwrap().parse().unwrap();
        adj[u] |= 1 << v;
        adj[v] |= 1 << u;
    }

    // 彩色数を求める
    let chromatic_number = find_chromatic_number(&adj, n);
    println!("{}", chromatic_number);
}

fn find_chromatic_number(adj: &Vec<u32>, n: usize) -> usize {
    let full_mask = (1u32 << n) - 1;

    // 最大独立集合のDPテーブル
    // dp[mask] = maskで表される頂点集合を最小の色数で彩色できるか
    let mut dp = vec![false; 1 << n];
    dp[0] = true;

    for colors in 1..=n {
        let mut new_dp = dp.clone();

        // 全ての独立集合を事前計算
        let mut independent_sets = Vec::new();
        for mask in 1u32..=(1 << n) - 1 {
            if is_independent_set(mask, adj, n) {
                independent_sets.push(mask);
            }
        }

        for mask in 0u32..(1 << n) {
            if !dp[mask as usize] {
                continue;
            }

            // maskに含まれていない頂点で独立集合を作る
            let remaining = full_mask ^ mask;

            for &ind_set in &independent_sets {
                if (ind_set & mask) == 0 && (ind_set & remaining) == ind_set {
                    new_dp[(mask | ind_set) as usize] = true;
                }
            }
        }

        if new_dp[full_mask as usize] {
            return colors;
        }

        dp = new_dp;
    }

    n
}

fn is_independent_set(mask: u32, adj: &Vec<u32>, n: usize) -> bool {
    for i in 0..n {
        if (mask & (1 << i)) != 0 {
            // 頂点iがmaskに含まれている場合、
            // iの隣接頂点がmaskに含まれていないかチェック
            if (mask & adj[i]) != 0 {
                return false;
            }
        }
    }
    true
}

// さらに高速化：包除原理を使った実装
fn find_chromatic_number_inclusion_exclusion(adj: &Vec<u32>, n: usize) -> usize {
    // 各頂点集合が独立集合かどうかを前計算
    let mut is_indep = vec![false; 1 << n];
    for mask in 0u32..(1 << n) {
        is_indep[mask as usize] = is_independent_set(mask, adj, n);
    }

    // 包除原理でk-彩色多項式を計算
    for k in 1..=n {
        let mut sum = 0i64;

        for mask in 0u32..(1 << n) {
            if !is_indep[mask as usize] {
                continue;
            }

            let vertices = mask.count_ones() as i32;
            let sign = if (n - vertices as usize) % 2 == 0 {
                1
            } else {
                -1
            };

            // k^vertices を計算
            let mut power = 1i64;
            for _ in 0..vertices {
                power *= k as i64;
                if power > 1_000_000_000 {
                    break;
                }
            }

            sum += sign as i64 * power;
        }

        if sum != 0 {
            return k;
        }
    }

    n
}

// 最適化版：より効率的なアルゴリズム
fn find_chromatic_number_optimized(adj: &Vec<u32>, n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    // 辺がない場合は1色
    let mut has_edge = false;
    for i in 0..n {
        if adj[i] != 0 {
            has_edge = true;
            break;
        }
    }
    if !has_edge {
        return 1;
    }

    // 完全グラフの場合はn色
    let mut is_complete = true;
    for i in 0..n {
        let expected = (1u32 << n) - 1 - (1u32 << i);
        if adj[i] != expected {
            is_complete = false;
            break;
        }
    }
    if is_complete {
        return n;
    }

    // 二部グラフかチェック
    let mut color = vec![-1i32; n];
    let mut is_bipartite = true;

    for start in 0..n {
        if color[start] != -1 {
            continue;
        }

        let mut stack = vec![start];
        color[start] = 0;

        while let Some(v) = stack.pop() {
            for u in 0..n {
                if (adj[v] & (1 << u)) != 0 {
                    if color[u] == -1 {
                        color[u] = 1 - color[v];
                        stack.push(u);
                    } else if color[u] == color[v] {
                        is_bipartite = false;
                        break;
                    }
                }
            }
            if !is_bipartite {
                break;
            }
        }
        if !is_bipartite {
            break;
        }
    }

    if is_bipartite {
        return 2;
    }

    // 一般的な場合：DPアプローチ
    find_chromatic_number(adj, n)
}
