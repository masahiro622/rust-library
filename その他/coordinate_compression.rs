/// 座標圧縮を行う関数
///
/// # 引数
/// - `vals`: 圧縮したい値の配列
///
/// # 戻り値
/// - `(compressed, mapping)`：
///   - `compressed` は圧縮後の配列
///   - `mapping` は元の値 → 圧縮後の値の対応表
pub fn coordinate_compression<T: Ord + Copy + std::hash::Hash>(
    vals: &[T],
) -> (Vec<usize>, rustc_hash::FxHashMap<T, usize>) {
    let mut sorted = vals.to_vec();
    sorted.sort_unstable();
    sorted.dedup();

    let map: rustc_hash::FxHashMap<T, usize> = sorted
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    let compressed = vals.iter().map(|&v| map[&v]).collect();

    (compressed, map)
}
