// requires: butterfly, butterfly_inv

pub fn convolution_blocked(a: &[Mint], b: &[Mint]) -> Vec<Mint> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }

    const BLOCK: usize = 1 << 22;
    const Z: usize = 1 << 23; // 998244353 で使える最大 2 冪長
    debug_assert!(2 * BLOCK - 1 <= Z);

    let mut res = vec![Mint::raw(0); a.len() + b.len() - 1];
    let iz = Mint::new(Z as u32).inv();

    // キャッシュ側を「ブロック数が少ない方」にしてメモリを抑える
    let pa = (a.len() + BLOCK - 1) / BLOCK;
    let pb = (b.len() + BLOCK - 1) / BLOCK;
    let (outer, inner) = if pa >= pb { (a, b) } else { (b, a) };

    let no = (outer.len() + BLOCK - 1) / BLOCK;
    let ni = (inner.len() + BLOCK - 1) / BLOCK;

    // inner 側を先に NTT して保持
    let mut inner_ntt: Vec<Vec<Mint>> = Vec::with_capacity(ni);
    let mut inner_len: Vec<usize> = Vec::with_capacity(ni);

    for i in 0..ni {
        let l = i * BLOCK;
        let r = (l + BLOCK).min(inner.len());
        let len = r - l;
        inner_len.push(len);

        let mut f = vec![Mint::raw(0); Z];
        f[..len].copy_from_slice(&inner[l..r]);
        butterfly(&mut f);
        inner_ntt.push(f);
    }

    let mut fo = vec![Mint::raw(0); Z];
    let mut prod = vec![Mint::raw(0); Z];

    for io in 0..no {
        let ol = io * BLOCK;
        let or = (ol + BLOCK).min(outer.len());
        let olen = or - ol;

        fo.fill(Mint::raw(0));
        fo[..olen].copy_from_slice(&outer[ol..or]);
        butterfly(&mut fo);

        for ii in 0..ni {
            let fi = &inner_ntt[ii];

            for k in 0..Z {
                prod[k] = fo[k] * fi[k];
            }
            butterfly_inv(&mut prod);

            let il = ii * BLOCK;
            let lim = olen + inner_len[ii] - 1;
            let off = ol + il;

            for k in 0..lim {
                res[off + k] += prod[k] * iz;
            }
        }
    }

    res
}
