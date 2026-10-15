pub struct SumMonoid;

impl Monoid for SumMonoid {
    type S = (i64, i64);

    fn identity() -> Self::S {
        (0, i64::MIN) // 単位元
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1.max(a.0 + b.1))
    }
}

pub struct Doubling<M: Monoid> {
    /// next[k][i]: 頂点 i から 2^k 回遷移した先の頂点
    next: Vec<Vec<usize>>,
    /// val[k][i]: 頂点 i から 2^k 回遷移する間に演算されたモノイドの値
    val: Vec<Vec<M::S>>,
}

impl<M: Monoid> Doubling<M> {
    /// 初期状態の遷移先(next_0)と、その1ステップの重み(val_0)、最大ステップ数(max_steps)から構築する
    pub fn new(next_0: Vec<usize>, val_0: Vec<M::S>, max_steps: usize) -> Self {
        let n = next_0.len();

        // max_steps に到達するために必要な最大の k (ビット長) を計算
        let mut log = 1;
        while (1usize << log) <= max_steps {
            log += 1;
        }

        let mut next = vec![vec![0; n]; log];
        let mut val = vec![vec![M::identity(); n]; log];

        // k = 0 (1ステップ) の初期化
        next[0][..n].copy_from_slice(&next_0[..n]);
        val[0][..n].clone_from_slice(&val_0[..n]);

        // k = 1 以降のテーブル構築 (ダブリングのコア部分)
        for k in 0..log - 1 {
            for i in 0..n {
                let next_node = next[k][i];
                next[k + 1][i] = next[k][next_node];
                // 前半の 2^k 歩の結果と、後半の 2^k 歩の結果を合成
                val[k + 1][i] = M::binary_operation(&val[k][i], &val[k][next_node]);
            }
        }

        Self { next, val }
    }

    /// 頂点 `start_state` から `steps` 回遷移した後の (最終到達頂点, モノイドの累積値) を返す
    pub fn query(&self, mut state: usize, mut steps: usize) -> (usize, M::S) {
        let mut res = M::identity();
        let mut k = 0;

        while steps > 0 {
            if steps & 1 == 1 {
                // 現在の状態から 2^k 歩進む
                res = M::binary_operation(&res, &self.val[k][state]);
                state = self.next[k][state];
            }
            steps >>= 1;
            k += 1;
        }

        (state, res)
    }
}
