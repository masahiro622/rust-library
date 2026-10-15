// ハンガリアン法　O(N^3)

use std::cmp::{max, min};
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut cost = vec![vec![0i64; n]; n];
    for i in 0..n {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        cost[i] = input
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
    }

    let (min_cost, assignment) = hungarian(&cost);
    let mut out = io::BufWriter::new(io::stdout());
    writeln!(out, "{}", min_cost).unwrap();
    for i in 0..n {
        write!(
            out,
            "{}{}",
            assignment[i],
            if i + 1 == n { "\n" } else { " " }
        )
        .unwrap();
    }
}

// Hungarian Algorithm for assignment problem (minimize total cost)
fn hungarian(cost: &[Vec<i64>]) -> (i64, Vec<usize>) {
    let n = cost.len();
    let mut u = vec![0; n + 1];
    let mut v = vec![0; n + 1];
    let mut p = vec![0; n + 1];
    let mut way = vec![0; n + 1];

    for i in 1..=n {
        let mut minv = vec![i64::MAX; n + 1];
        let mut used = vec![false; n + 1];
        p[0] = i;
        let mut j0 = 0;
        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = i64::MAX;
            let mut j1 = 0;
            for j in 1..=n {
                if !used[j] {
                    let cur = cost[i0 - 1][j - 1] - u[i0] - v[j];
                    if cur < minv[j] {
                        minv[j] = cur;
                        way[j] = j0;
                    }
                    if minv[j] < delta {
                        delta = minv[j];
                        j1 = j;
                    }
                }
            }
            for j in 0..=n {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }
            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }
        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 {
                break;
            }
        }
    }

    let mut ans = vec![0; n];
    for j in 1..=n {
        if p[j] > 0 {
            ans[p[j] - 1] = j - 1;
        }
    }
    (-v[0], ans)
}
