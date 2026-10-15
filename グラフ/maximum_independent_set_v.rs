// 最大独立集合を求める　O(n^2 * 2^(n/2))
use std::collections::HashMap;

pub fn maximum_independent_set(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut adj = vec![0u64; n];
    for &(u, v) in edges {
        adj[u] |= 1 << v;
        adj[v] |= 1 << u;
    }

    let n1 = n / 2;
    let n2 = n - n1;

    // 前半部分の独立集合全列挙
    let mut left = vec![];
    for mask in 0..1 << n1 {
        let mut ok = true;
        let bits = mask_to_u64(mask, n1);
        for i in 0..n1 {
            if (mask >> i) & 1 == 1 {
                // 自分以外の選択済み頂点と隣接していないか
                if (adj[i] & (bits ^ (1 << i))) != 0 {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            left.push((mask, mask.count_ones() as usize));
        }
    }

    // 後半部分の独立集合と最大サイズをマップに記録
    let mut right_map = HashMap::new();
    for mask in 0u64..1u64 << n2 {
        let mut ok = true;
        let mut bits = 0u64;
        for i in 0..n2 {
            if (mask >> i) & 1 == 1 {
                let real = i + n1;
                if (adj[real] & bits) != 0 {
                    ok = false;
                    break;
                }
                bits |= 1 << real;
            }
        }
        if ok {
            right_map.insert(bits, mask.count_ones() as usize);
        }
    }
    // 右側集合で互いに被らないものを用いて最大サイズ
    let mut max_size = 0;
    let mut best_mask = 0u64;
    for &(lmask, lsize) in &left {
        let lbits = mask_to_u64(lmask, n1);
        let mut conflict = 0u64;
        for i in 0..n1 {
            if (lbits >> i) & 1 == 1 {
                conflict |= adj[i];
            }
        }
        for (&rbits, &rsize) in &right_map {
            if (lbits & rbits) == 0 && (conflict & rbits) == 0 {
                if lsize + rsize > max_size {
                    max_size = lsize + rsize;
                    best_mask = lbits | rbits;
                }
            }
        }
    }

    // best_mask に対応する頂点集合を抽出
    let result = (0..n).filter(|&i| (best_mask >> i) & 1 == 1).collect();
    result
}
// helper: bitmask を u64 に変換
fn mask_to_u64(mask: usize, bits: usize) -> u64 {
    (0..bits)
        .filter(|&i| (mask >> i) & 1 == 1)
        .map(|i| 1 << i)
        .sum()
}
