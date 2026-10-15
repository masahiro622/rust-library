fn conversion_basis_from_monomial_to_newton(&self, p: &[Mint]) -> Vec<Mint> {
    let n = self.len();
    if n == 0 {
        return vec![];
    }
    let b = n.next_power_of_two();
    let tree = Self::build_subproduct_tree(p);
    let mut tree_inv = vec![FPS(vec![]); b * 2];
    for i in 1..b {
        let right_n = tree[i * 2 + 1].len();
        let max_q_len = right_n.saturating_sub(1);
        if max_q_len > 0 {
            let rg_len = tree[i * 2].len();
            let mut rg = tree[i * 2].clone().rev().resized(rg_len.max(max_q_len));
            rg.truncate(max_q_len);
            tree_inv[i * 2] = rg.inv();
        }
    }
    let mut res = vec![Mint::raw(0); n];
    fn rec(
        m: &[FPS],
        m_inv: &[FPS],
        id: usize,
        l: usize,
        r: usize,
        g: &FPS,
        res: &mut [Mint],
        n: usize,
    ) {
        if n <= l {
            return;
        }
        if r - l == 1 {
            res[l] = g.evaluate(Mint::raw(0));
            return;
        }
        let mid = (l + r) / 2;
        let m_l = &m[id * 2];
        let n_g = g.len();
        let n_m = m_l.len();
        let quo = if n_g < n_m {
            FPS(vec![])
        } else {
            let deg = n_g - n_m + 1;
            let mut f_rev: Vec<Mint> = g.0.iter().rev().copied().collect();
            f_rev.truncate(deg);
            let inv_rg = &m_inv[id * 2].0;
            let mut q_vec = FPS(convolution(
                &f_rev,
                if inv_rg.len() > deg {
                    &inv_rg[..deg]
                } else {
                    inv_rg
                },
            ));
            q_vec.truncate(deg);
            q_vec.rev().trim_trailing_zeros()
        };
        let mut rem = g - &(m_l * &quo);
        rem = rem.trim_trailing_zeros();
        rec(m, m_inv, id * 2, l, mid, &rem, res, n);
        rec(m, m_inv, id * 2 + 1, mid, r, &quo, res, n);
    }
    rec(&tree, &tree_inv, 1, 0, b, self, &mut res, n);
    res
}
