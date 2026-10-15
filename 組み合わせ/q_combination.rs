// requires: mint.rs, comb.rs
// q-ary Lucas theorem
// max_n <= modulus - 1
pub struct QComb {
    fact: Vec<Mint>,
    inv_fact: Vec<Mint>,
    qfact: Vec<Mint>,
    qinv_qfact: Vec<Mint>,
    max_n: usize,
    l: usize,
}

impl QComb {
    pub fn new(n: usize, q: Mint) -> Self {
        if q == Mint::raw(1) {
            let comb = Comb::new(n);
            return Self {
                fact: comb.fact,
                inv_fact: comb.inv_fact,
                max_n: comb.max_n,
                qfact: vec![Mint::raw(1)],
                qinv_qfact: vec![Mint::raw(1)],
                l: 1,
            };
        }

        if q == Mint::raw(0) {
            return Self {
                fact: vec![Mint::raw(1); n + 1],
                inv_fact: vec![Mint::raw(1); n + 1],
                qfact: vec![Mint::raw(1)],
                qinv_qfact: vec![Mint::raw(1)],
                max_n: n,
                l: 1,
            };
        }

        let mut l = n + 1;
        let mut current_q = q;
        for i in 1..=n {
            if current_q == Mint::raw(1) {
                l = i;
                break;
            }
            current_q *= q;
        }

        let comb = Comb::new(n / l);

        let mut qpow_vec = vec![Mint::raw(1); l];
        let mut qfact = vec![Mint::raw(1); l];
        let mut qpow = Mint::raw(1);

        for i in 1..l {
            qpow *= q;
            qpow_vec[i] = qpow;
            qfact[i] = qfact[i - 1] * (Mint::raw(1) - qpow);
        }

        let mut qinv_qfact = vec![Mint::raw(1); l];
        if l > 1 {
            qinv_qfact[l - 1] = qfact[l - 1].inv();
            for i in (1..l).rev() {
                qinv_qfact[i - 1] = qinv_qfact[i] * (Mint::raw(1) - qpow_vec[i]);
            }
        }

        Self {
            fact: comb.fact,
            inv_fact: comb.inv_fact,
            max_n: comb.max_n,
            qfact,
            qinv_qfact,
            l,
        }
    }

    pub fn combination(&self, n: usize, k: usize) -> Mint {
        assert!(n <= self.max_n);
        if k > n {
            return Mint::raw(0);
        }

        let n1 = n / self.l;
        let k1 = k / self.l;
        let n0 = n % self.l;
        let k0 = k % self.l;

        if n0 < k0 {
            return Mint::raw(0);
        }

        let lucas_part = self.fact[n1] * self.inv_fact[k1] * self.inv_fact[n1 - k1];
        let q_part = self.qfact[n0] * self.qinv_qfact[k0] * self.qinv_qfact[n0 - k0];

        lucas_part * q_part
    }
}
