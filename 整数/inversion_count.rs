pub fn inversion_count<T: Ord>(a: &[T]) -> usize {
    // 座標圧縮
    let mut b: Vec<&T> = a.iter().collect();
    b.sort_unstable();
    b.dedup_by(|x, y| *x == *y);

    let mut compressed = Vec::with_capacity(a.len());
    for x in a {
        let idx = b.binary_search(&x).unwrap();
        compressed.push(idx);
    }

    // 転倒数の計算
    let mut ft = ac_library::FenwickTree::new(b.len(), 0usize);
    let mut count = 0usize;
    for &x in compressed.iter().rev() {
        count += ft.sum(..x);
        ft.add(x, 1);
    }
    count
}
