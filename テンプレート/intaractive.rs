// interactive用のテンプレ
#[allow(unused_imports)]
use {
    ac_library::{ModInt998244353 as Mint, *},
    itertools::{iproduct, Itertools},
    proconio::{input, marker::*, source::line::LineSource},
    std::collections::*,
    std::io::BufReader,
};
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    let stdin = std::io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin));

    input! {
        from &mut source,
        n: usize

    }
    //以下input!には from &mut source, を付ける
}
