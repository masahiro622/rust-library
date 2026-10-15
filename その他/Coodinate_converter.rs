/// 2次元の座標(x, y)と1次元のインデックスを相互変換する構造体
pub struct CoordConverter {
    pub h: usize,
    pub w: usize,
}

impl CoordConverter {
    /// 新しいコンバーターを作成します
    pub fn new(h: usize, w: usize) -> Self {
        Self { h, w }
    }

    pub fn pack(&self, x: usize, y: usize) -> usize {
        x * self.w + y
    }

    pub fn unpack(&self, index: usize) -> (usize, usize) {
        (index / self.w, index % self.w)
    }
}

pub struct CoordConverter3D {
    pub d1: usize,
    pub d2: usize,
    pub d3: usize,
}

impl CoordConverter3D {
    pub fn new(d1: usize, d2: usize, d3: usize) -> Self {
        Self { d1, d2, d3 }
    }

    /// 3次元座標(x, y, z)を1次元インデックスにまとめる
    pub fn pack(&self, x: usize, y: usize, z: usize) -> Option<usize> {
        if x < self.d1 && y < self.d2 && z < self.d3 {
            // 計算の工夫: x * (d2 * d3) + y * d3 + z は
            // (x * d2 + y) * d3 + z と書くと掛け算の回数が減ります
            Some((x * self.d2 + y) * self.d3 + z)
        } else {
            None
        }
    }

    /// 1次元インデックスを3次元座標(x, y, z)に分解する
    pub fn unpack(&self, mut index: usize) -> Option<(usize, usize, usize)> {
        let total_size = self.d1 * self.d2 * self.d3;
        if index < total_size {
            let z = index % self.d3;
            index /= self.d3;
            let y = index % self.d2;
            index /= self.d2;
            let x = index;
            Some((x, y, z))
        } else {
            None
        }
    }
}

pub struct NdCoordConverter {
    /// 各次元のサイズ [d1, d2, d3, ..., dn]
    pub shape: Vec<usize>,
}

impl NdCoordConverter {
    pub fn new(shape: Vec<usize>) -> Self {
        Self { shape }
    }

    /// N次元座標を1次元インデックスにまとめる
    pub fn pack(&self, coords: &[usize]) -> Option<usize> {
        if coords.len() != self.shape.len() {
            return None; // 次元数が一致しない
        }

        let mut index = 0;
        for (i, &c) in coords.iter().enumerate() {
            if c >= self.shape[i] {
                return None; // 範囲外
            }
            index = index * self.shape[i] + c;
        }
        Some(index)
    }

    /// 1次元インデックスをN次元座標に分解する
    pub fn unpack(&self, mut index: usize) -> Option<Vec<usize>> {
        // 全要素数を計算
        let total_size: usize = self.shape.iter().product();
        if index >= total_size {
            return None; // 範囲外
        }

        let mut coords = vec![0; self.shape.len()];
        // 後ろの次元から逆順に計算していく
        for i in (0..self.shape.len()).rev() {
            let dim = self.shape[i];
            coords[i] = index % dim;
            index /= dim;
        }
        Some(coords)
    }
}
