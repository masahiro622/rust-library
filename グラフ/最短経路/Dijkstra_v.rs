const INF: usize = usize::MAX / 2;

struct Dijkstra {
    distance: Vec<usize>,
    parent: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edge: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edge: Vec<Vec<(usize, usize)>>, init: usize) -> Self {
        let mut distance = vec![INF; n];
        let mut parent = vec![INF; n];
        let mut heap = RadixHeap::new();
        heap.push(0, init);

        while let Some((dist, target)) = heap.pop() {
            if distance[target] < dist {
                // 既にもっと短い経路が見つかってるので無視
                continue;
            }
            for &(next, cost) in &edge[target] {
                if distance[next] > dist + cost {
                    // 短い経路の候補となるので処理を行う
                    distance[next] = dist + cost;
                    heap.push(distance[next], next);
                    // ひとつ前の頂点を保存しておく
                    parent[next] = target;
                }
            }
        }
        Self { distance, parent }
    }

    /// 頂点への最短距離を返す。到達不可能な場合は None を返す。
    pub fn distance(&self, target: usize) -> Option<usize> {
        let dist = self.distance[target];
        if dist == INF {
            None
        } else {
            Some(dist)
        }
    }

    /// 頂点への経路を返す。到達不可能な場合は None を返す。
    pub fn get_path(&self, target: usize) -> Option<Vec<usize>> {
        // 到達不可能な場合は None
        if self.distance[target] == INF {
            return None;
        }

        let mut current = target;
        let mut res = vec![current];
        while self.parent[current] != INF {
            let next = self.parent[current];
            res.push(next);
            current = next;
        }
        res.reverse();
        Some(res)
    }
}

#[derive(Clone, Debug)]
pub struct RadixHeap<T> {
    last: usize,
    size: usize,
    // usizeのビット長(64) + 1 = 65個のバケットが必要
    buckets: [Vec<(usize, T)>; 65],
}

impl<T> RadixHeap<T> {
    /// 空のRadix Heapを生成します
    pub fn new() -> Self {
        Self {
            last: 0,
            size: 0,
            // 65個の空ベクタで初期化
            buckets: std::array::from_fn(|_| Vec::new()),
        }
    }

    /// 距離の差分から格納すべきバケットのインデックスを計算します
    #[inline]
    fn get_bucket(last: usize, dist: usize) -> usize {
        let diff = dist ^ last;
        if diff == 0 {
            0
        } else {
            // 最上位ビット(MSB)の位置からバケットを決定
            // 64 - leading_zeros で必要なビット幅が求まる
            64 - diff.leading_zeros() as usize
        }
    }

    /// 要素をヒープに挿入します
    pub fn push(&mut self, dist: usize, val: T) {
        // Radix Heapの制約: 最後にpopした値(last)より小さい値はpushできない
        assert!(dist >= self.last, "Radix Heap requires monotonic keys");

        let b = Self::get_bucket(self.last, dist);
        self.buckets[b].push((dist, val));
        self.size += 1;
    }

    /// 最小要素を取り出します
    pub fn pop(&mut self) -> Option<(usize, T)> {
        if self.size == 0 {
            return None;
        }

        // バケット0（最小値のバケット）が空の場合、再分配を行う
        if self.buckets[0].is_empty() {
            // 要素が存在する最もインデックスの小さいバケットを探す
            let mut i = 1;
            while self.buckets[i].is_empty() {
                i += 1;
            }

            // そのバケット内の最小値を探す
            let mut min_dist = usize::MAX;
            for &(dist, _) in &self.buckets[i] {
                if dist < min_dist {
                    min_dist = dist;
                }
            }

            // 最小値の基準を更新
            self.last = min_dist;

            // バケット内の要素を新しい基準(last)に従って再分配する
            // バケットの所有権を一時的に奪ってイテレートする
            let old_bucket = std::mem::take(&mut self.buckets[i]);
            for (dist, val) in old_bucket {
                let b = Self::get_bucket(self.last, dist);
                self.buckets[b].push((dist, val));
            }
        }

        self.size -= 1;
        // バケット0から要素を取り出す（同じ距離なら順序は問わない）
        self.buckets[0].pop()
    }

    /// ヒープが空かどうかを返します
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// ヒープ内の要素数を返します
    pub fn len(&self) -> usize {
        self.size
    }
}

impl<T> Default for RadixHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}
