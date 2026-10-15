fn main() {
    let n = 5;
    for mask in 0..(1 << n) {
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                // i 番目を選んでいる
            }
        }
    }
}

// 部分集合を列挙する関数
fn subset(mask: usize, n: usize) -> Vec<usize> {
    (0..n).filter(|&i| (mask >> i) & 1 == 1).collect()
}

let n = 4;
for mask in 0..(1 << n) {
    println!("{:?}", subset(mask, n));
}

// mask != 0 の場合
for mask in 1..(1 << n) {
    // mask != 0
}

// ちょうど k 個選ぶ場合
for mask in 0..(1 << n) {
    if mask.count_ones() == k {
        // k 個ちょうど選ばれている
    }
}

// 部分集合の和を求める関数
fn subset_sums(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut res = Vec::new();
    for mask in 0..(1 << n) {
        let mut sum = 0;
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                sum += a[i];
            }
        }
        res.push(sum);
    }
    res
}

// mask の部分集合を全て列挙する
let mask = 0b101101;
let mut sub = mask;
while sub > 0 {
    println!("{:b}", sub);
    sub = (sub - 1) & mask;
}

// ビット反転
let mask = 0b1011;
let n = 4;
let complement = (!mask) & ((1 << n) - 1); // n ビットに制限
println!("{:b}", complement); // 0100

// 2進数で一番右の 1 を取得
let mask = 0b1011000;
let lowbit = mask & (!mask + 1); // 2進数で一番右の 1 だけ残る
println!("{:b}", lowbit); // 1000


// ちょうど k 個選ぶ場合の次の組み合わせを求める関数
fn next_combination(mut x: usize) -> Option<usize> {
    let u = x & (!x + 1);
    let v = u + x;
    if v == 0 { return None; }
    Some(v + (((v ^ x) / u) >> 2))
}

let n = 5;
let k = 3;
let mut mask = (1 << k) - 1;
while mask < (1 << n) {
    println!("{:05b}", mask); // k 個選ばれたビット
    if let Some(next) = next_combination(mask) {
        mask = next;
    } else {
        break;
    }
}

const INF: usize = 1 << 60;

/// BitDPを用いて巡回セールスマン問題(TSP)を解く関数
///
/// # 引数
/// * `dist` - 頂点間の距離行列。`dist[u][v]` は u から v への距離。
///
/// 道が存在しない場合は `INF` を指定してください。
///
/// # 戻り値
/// すべての頂点を1度ずつ訪問し、開始点に戻るための最小コスト。
/// 経路が存在しない場合は `INF` を返します。
pub fn solve_tsp(dist: &[Vec<usize>]) -> usize {
    let n = dist.len();
    if n == 0 || n == 1 {
        return 0;
    }

    // dp[S][v] : 訪問済みの頂点集合が S (ビットマスク) で、現在頂点 v にいるときの最小コスト
    let mut dp = vec![vec![INF; n]; 1 << n];

    // 頂点0を始点とする (どの頂点から始めても閉路のコストは同じ)
    // 頂点0のみを訪問済み(1 << 0 = 1)とし、現在位置は頂点0
    dp[1][0] = 0;

    for s in 1..(1 << n) {
        for u in 0..n {
            // 現在の状態に到達不可能であればスキップ
            if dp[s][u] == INF {
                continue;
            }

            for v in 0..n {
                // 頂点vが未訪問の場合のみ遷移
                if (s & (1 << v)) == 0 {
                    let next_s = s | (1 << v);
                    // 頂点uから頂点vへの経路が存在する場合のみ更新
                    if dist[u][v] != INF {
                        dp[next_s][v] = dp[next_s][v].min(dp[s][u] + dist[u][v]);
                    }
                }
            }
        }
    }

    // すべての頂点を訪問した状態
    let all_visited = (1 << n) - 1;
    let mut min_cost = INF;

    // 最後に任意の頂点 u から始点 0 へ戻るコストを加算して最小値を探す
    for u in 0..n {
        if dp[all_visited][u] != INF && dist[u][0] != INF {
            min_cost = min_cost.min(dp[all_visited][u] + dist[u][0]);
        }
    }

    min_cost
}
