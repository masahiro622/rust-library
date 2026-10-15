/// 到達不能を表す無限大の値（オーバーフロー防止のため i64::MAX / 2 に設定）
pub const INF: i64 = i64::MAX / 2;

/// ジョンソンのアルゴリズムによる全点対最短経路
///
/// 負の閉路が存在する場合は `None` を返す。
/// 存在しない場合は `Some(dist)` を返し、dist[u][v] は頂点 u から v への最短距離。
/// 到達できない場合は `INF` となる。
///
/// - `n`: 頂点数
/// - `edges`: (u, v, weight) の有向辺のリスト
pub fn johnson(n: usize, edges: &[(usize, usize, i64)]) -> Option<Vec<Vec<i64>>> {
    // グラフの隣接リストを構築
    let mut adj = vec![vec![]; n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
    }

    // ステップ1: ベルマン・フォード法によるポテンシャル h の計算
    // 超頂点から全頂点へ重み0の辺を張った状態と等価にするため、初期値はすべて0
    let mut h = vec![0; n];
    let mut updated = false;

    // 頂点数 n 回の反復。n回目で更新があれば負の閉路が存在
    for _ in 0..n {
        updated = false;
        for u in 0..n {
            for &(v, w) in &adj[u] {
                // h[u] はすべて0から始まるため INF の判定は不要
                if h[v] > h[u] + w {
                    h[v] = h[u] + w;
                    updated = true;
                }
            }
        }
        if !updated {
            break; // 早期終了
        }
    }

    // n回目のループで更新があった場合は負の閉路が存在
    if updated {
        return None;
    }

    // ステップ2: 辺の再重み付け
    // w'(u, v) = w(u, v) + h[u] - h[v]
    let mut reweighted_adj = vec![vec![]; n];
    for u in 0..n {
        for &(v, w) in &adj[u] {
            let new_weight = w + h[u] - h[v];
            reweighted_adj[u].push((v, new_weight));
        }
    }

    // ステップ3 & 4: 各頂点を始点とするダイクストラ法と距離の復元
    let mut dist_matrix = vec![vec![INF; n]; n];

    // アロケーションコスト削減のため、BinaryHeap は1つだけ生成して使い回す
    let mut heap: RadixHeap<usize> = RadixHeap::new();

    for s in 0..n {
        let dist = &mut dist_matrix[s];
        dist[s] = 0;
        heap.clear(); // 内部バッファを再利用
        heap.push(0, s);

        while let Some((d, u)) = heap.pop() {
            // すでに短い経路が見つかっている場合はスキップ
            if d > dist[u] as usize {
                continue;
            }

            for &(v, w) in &reweighted_adj[u] {
                if dist[v] > dist[u] + w {
                    dist[v] = dist[u] + w;
                    heap.push(dist[v] as usize, v);
                }
            }
        }

        // 距離の復元: d(u, v) = d'(u, v) - h[u] + h[v]
        for v in 0..n {
            if dist[v] != INF {
                dist[v] = dist[v] - h[s] + h[v];
            }
        }
    }

    Some(dist_matrix)
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

    /// ヒープを空にします（内部バッファは再利用されるため、アロケーションコストは発生しません）
    pub fn clear(&mut self) {
        self.last = 0;
        self.size = 0;
        for bucket in &mut self.buckets {
            bucket.clear();
        }
    }
}

impl<T> Default for RadixHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}
