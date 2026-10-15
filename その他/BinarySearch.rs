pub trait BinarySearch<T> {
    fn lower_bound(&self, key: T) -> usize;
    fn upper_bound(&self, key: T) -> usize;
}

impl<T> BinarySearch<T> for [T]
where
    T: Ord,
{
    fn lower_bound(&self, key: T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if key <= self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }

    fn upper_bound(&self, key: T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if key < self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}

/// 二分探索 (ok, ng スタイル)
/// - `ok`: 条件を満たす初期値
/// - `ng`: 条件を満たさない初期値
/// - `pred(x)` が true になる最大/最小を探索する
pub fn binary_search<F>(mut ok: isize, mut ng: isize, pred: F) -> isize
where
    F: Fn(isize) -> bool,
{
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if pred(mid) {
            ok = mid; // 成立する側に寄せる
        } else {
            ng = mid; // 成立しない側に寄せる
        }
    }
    ok
}

pub fn binary_search_f64<F>(mut ok: f64, mut ng: f64, pred: F) -> f64
where
    F: Fn(f64) -> bool,
{
    for _ in 0..100 {
        let mid = (ok + ng) / 2.0;
        if pred(mid) {
            ok = mid; // 成立する側に寄せる
        } else {
            ng = mid; // 成立しない側に寄せる
        }
    }
    (ok + ng) / 2.0
}
