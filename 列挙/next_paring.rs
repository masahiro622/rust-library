// [0,1],[2,3],[4,5]　がペア
fn next_pairing(p: &mut [usize]) -> bool {
    let n = p.len();
    let mut used = 0_u32;
    for i in (0..n).rev() {
        used |= 1 << p[i];
        if i % 2 == 1 && p[i] + 1 < used.ilog2() as usize {
            p[i] = (used >> (p[i] + 1)).trailing_zeros() as usize + p[i] + 1;
            used ^= 1 << p[i];
            for i in i + 1..n {
                p[i] = used.trailing_zeros() as usize;
                used ^= 1 << p[i];
            }
            return true;
        }
    }
    false
}
