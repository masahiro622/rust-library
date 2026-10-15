use proconio::{fastout, input};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::{Duration, Instant};
#[fastout]
fn main() {
    let start = Instant::now();
    let time_limit = Duration::from_millis(1900); // 1.9秒で止める
    let mut rng = rand::thread_rng(); // グローバルな乱数生成器を使用
    while start.elapsed() < time_limit {
        //let random=rng.gen_range(1..=n);で乱数生成
    }
}
