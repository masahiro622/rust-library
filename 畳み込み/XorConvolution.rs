fn xor_convolution(a: &mut Vec<Mint>, b: &mut [Mint]) -> Vec<Mint> {
    fn fwht_xor(f: &mut [Mint]) {
        let n = f.len();
        let mut len = 1;
        for _ in 0..n.trailing_zeros() {
            for i in (0..n).step_by(len * 2) {
                for j in i..i + len {
                    let x = f[j];
                    let y = f[j + len];
                    f[j] = x + y;
                    f[j + len] = x - y;
                }
            }
            len *= 2;
        }
    }
    let n = a.len();
    fwht_xor(a);
    fwht_xor(b);
    for i in 0..n {
        a[i] *= b[i];
    }
    fwht_xor(a);
    let inv_n = Mint::new(n).inv();
    for x in a.iter_mut() {
        *x *= inv_n;
    }
    std::mem::take(a)
}
