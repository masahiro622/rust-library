#[derive(Debug)]
struct PrefixSum2D {
    s: Vec<Vec<i32>>,
}

impl PrefixSum2D {
    fn new(a: &[Vec<i32>]) -> Self {
        let h = a.len();
        let w = a[0].len();
        let mut s = vec![vec![0; w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                s[i + 1][j + 1] = s[i][j + 1] + s[i + 1][j] - s[i][j] + a[i][j];
            }
        }
        Self { s }
    }

    // [x1, x2) x [y1, y2) の矩形和
    fn query(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
        self.s[x2][y2] - self.s[x1][y2] - self.s[x2][y1] + self.s[x1][y1]
    }
}
