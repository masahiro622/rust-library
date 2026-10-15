use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
//集合の一致を確認するためのZobristハッシュを用いた集合の実装
struct ZobristSet {
    hash_map: HashMap<usize, u64>, // 要素に対応する乱数
}

impl ZobristSet {
    // seed は内部で現在時刻から生成
    fn new(universe: &[usize]) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("SystemTime before UNIX_EPOCH");
        // 秒 + ナノ秒を混ぜて u64 seed にする（ナノ秒だけより衝突しにくい）
        let seed: u64 = now.as_secs() ^ ((now.subsec_nanos() as u64) << 32);

        Self::new_with_seed(universe, seed)
    }

    fn new_with_seed(universe: &[usize], seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut hash_map = HashMap::new();
        for &elem in universe {
            hash_map.insert(elem, rng.r#gen());
        }
        ZobristSet { hash_map }
    }

    fn hash(&self, set: &std::collections::HashSet<usize>) -> u64 {
        set.iter()
            .map(|x| self.hash_map[x])
            .fold(0, |acc, x| acc ^ x)
    }
}
