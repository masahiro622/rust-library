use ModInt998244353 as Mint;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FPS(pub Vec<Mint>);

impl std::ops::Deref for FPS {
    type Target = Vec<Mint>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FPS {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Mint>> for FPS {
    fn from(v: Vec<Mint>) -> Self {
        FPS(v)
    }
}

impl FPS {
    fn zip_with(&self, rhs: &Self, op: impl Fn(Mint, Mint) -> Mint) -> FPS {
        let n = self.len().max(rhs.len());
        let mut c = vec![Mint::raw(0); n];
        for i in 0..n {
            let x = if i < self.len() {
                self[i]
            } else {
                Mint::raw(0)
            };
            let y = if i < rhs.len() { rhs[i] } else { Mint::raw(0) };
            c[i] = op(x, y);
        }
        FPS(c)
    }
}

impl std::ops::Add for FPS {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::Add for &FPS {
    type Output = FPS;
    fn add(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, |x, y| x + y)
    }
}

impl std::ops::Sub for &FPS {
    type Output = FPS;
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, |x, y| x - y)
    }
}

impl std::ops::Sub for FPS {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl std::ops::Mul for &FPS {
    type Output = FPS;
    fn mul(self, rhs: Self) -> Self::Output {
        FPS(convolution(&self.0, &rhs.0))
    }
}

impl std::ops::Mul for FPS {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl std::ops::Mul<Mint> for &FPS {
    type Output = FPS;
    fn mul(self, rhs: Mint) -> Self::Output {
        FPS(self.iter().map(|&x| x * rhs).collect())
    }
}

impl std::ops::Mul<Mint> for FPS {
    type Output = Self;
    fn mul(self, rhs: Mint) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::Div<Mint> for &FPS {
    type Output = FPS;
    fn div(self, rhs: Mint) -> Self::Output {
        let inv = rhs.inv();
        FPS(self.iter().map(|&x| x * inv).collect())
    }
}

impl std::ops::Div<Mint> for FPS {
    type Output = Self;
    fn div(self, rhs: Mint) -> Self::Output {
        &self / rhs
    }
}

impl std::ops::Div for &FPS {
    type Output = FPS;
    fn div(self, rhs: Self) -> Self::Output {
        let mut g_vec = rhs.clone();
        let m = g_vec.iter().rposition(|&x| x != Mint::raw(0)).unwrap_or(0) + 1;
        g_vec.truncate(m);

        assert!(m > 0, "Division by zero polynomial");

        if m == 1 {
            return self * g_vec[0].inv();
        }

        let n = self.len();
        if n < m {
            return FPS(Vec::new());
        }

        let deg = n - m + 1;

        let mut q = &self.clone().rev() * &g_vec.rev().resized(deg).inv();

        q.truncate(deg);
        q.rev().trim_trailing_zeros()
    }
}

impl std::ops::Div for FPS {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

impl std::ops::Rem for &FPS {
    type Output = FPS;
    fn rem(self, rhs: Self) -> Self::Output {
        let q = self / rhs;
        (self - &(&q * rhs)).trim_trailing_zeros()
    }
}

impl std::ops::Rem for FPS {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        &self % &rhs
    }
}

impl std::ops::AddAssign for FPS {
    fn add_assign(&mut self, rhs: Self) {
        *self = &*self + &rhs;
    }
}
impl std::ops::SubAssign for FPS {
    fn sub_assign(&mut self, rhs: Self) {
        *self = &*self - &rhs;
    }
}
impl std::ops::MulAssign for FPS {
    fn mul_assign(&mut self, rhs: Self) {
        *self = &*self * &rhs;
    }
}
impl std::ops::DivAssign for FPS {
    fn div_assign(&mut self, rhs: Self) {
        *self = &*self / &rhs;
    }
}
impl std::ops::RemAssign for FPS {
    fn rem_assign(&mut self, rhs: Self) {
        *self = &*self % &rhs;
    }
}
impl std::ops::MulAssign<Mint> for FPS {
    fn mul_assign(&mut self, rhs: Mint) {
        for x in &mut self.0 {
            *x *= rhs;
        }
    }
}
impl std::ops::DivAssign<Mint> for FPS {
    fn div_assign(&mut self, rhs: Mint) {
        *self = &*self / rhs;
    }
}

impl FPS {
    pub fn new(v: Vec<Mint>) -> Self {
        FPS(v)
    }

    pub fn rev(mut self) -> Self {
        self.0.reverse();
        self
    }

    pub fn resized(mut self, new_len: usize) -> Self {
        self.0.resize(new_len, Mint::raw(0));
        self
    }

    pub fn evaluate(&self, x: Mint) -> Mint {
        let mut res = Mint::raw(0);
        for &coef in self.iter().rev() {
            res *= x;
            res += coef;
        }
        res
    }

    pub fn factorials(n: usize) -> (Vec<Mint>, Vec<Mint>) {
        let mut fact = vec![Mint::raw(1); n];
        let mut inv_fact = vec![Mint::raw(1); n];
        for i in 1..n {
            fact[i] = fact[i - 1] * Mint::new(i);
        }
        if n > 0 {
            inv_fact[n - 1] = fact[n - 1].inv();
            for i in (1..n).rev() {
                inv_fact[i - 1] = inv_fact[i] * Mint::new(i);
            }
        }
        (fact, inv_fact)
    }

    #[allow(clippy::manual_is_multiple_of)]
    pub fn sign(n: usize) -> Mint {
        if n % 2 == 0 {
            Mint::raw(1)
        } else {
            -Mint::raw(1)
        }
    }

    fn legendre_symbol(a: Mint) -> Mint {
        a.pow((Mint::modulus() as u64 - 1) / 2)
    }

    fn mod_sqrt(n: Mint) -> Option<Mint> {
        if n == Mint::raw(0) {
            return Some(Mint::raw(0));
        }
        if FPS::legendre_symbol(n) != Mint::raw(1) {
            return None;
        }
        let mut q = Mint::modulus() as u64 - 1;
        let mut s = 0u32;
        while q & 1 == 0 {
            q >>= 1;
            s += 1;
        }
        let mut z = Mint::raw(2);
        while FPS::legendre_symbol(z) != -Mint::raw(1) {
            z += Mint::raw(1);
        }
        let mut c = z.pow(q);
        let mut r = n.pow((q + 1) >> 1);
        let mut t = n.pow(q);
        let mut m = s;
        while t != Mint::raw(1) {
            let mut i = 1u32;
            let mut t2i = t * t;
            while t2i != Mint::raw(1) {
                t2i *= t2i;
                i += 1;
                if i == m {
                    return None;
                }
            }
            let b = c.pow(1 << (m - i - 1));
            r *= b;
            t *= b * b;
            c = b * b;
            m = i;
        }
        Some(r)
    }

    pub fn trim_trailing_zeros(&mut self) -> FPS {
        while let Some(&last) = self.0.last() {
            if last == Mint::raw(0) {
                self.0.pop();
            } else {
                break;
            }
        }
        std::mem::take(self)
    }

    pub fn all_prod(polys: &mut Vec<FPS>) -> FPS {
        if polys.is_empty() {
            return FPS(vec![Mint::raw(1)]);
        }
        for p in polys.iter_mut() {
            *p = p.trim_trailing_zeros();
            if p.is_empty() {
                return FPS(vec![Mint::raw(0)]);
            }
        }

        let mut k = 16;
        while polys.len() > 1 {
            let mut pos = polys.len();
            let mut i = 0;

            while i < polys.len() {
                if pos == polys.len() || polys[pos].len() + polys[i].len() - 1 > k {
                    pos = i;
                    i += 1;
                } else {
                    let b = polys.swap_remove(i);
                    let a = std::mem::take(&mut polys[pos]);

                    polys[pos] = &a * &b;
                }
            }
            k *= 2;
        }

        polys[0].clone()
    }

    pub fn derivative(&self) -> FPS {
        if self.is_empty() {
            return FPS(vec![]);
        }
        let mut res = vec![Mint::raw(0); self.len() - 1];
        for i in 1..self.len() {
            res[i - 1] = self[i] * Mint::new(i);
        }
        FPS(res)
    }

    pub fn integral(&self) -> FPS {
        if self.is_empty() {
            return FPS(vec![]);
        }
        let mut res = vec![Mint::raw(0); self.len() + 1];
        for i in 0..self.len() {
            res[i + 1] = self[i] * Mint::new(i + 1).inv();
        }
        FPS(res)
    }

    pub fn inv(&self) -> FPS {
        let n = self.len();
        if n == 0 {
            return FPS(vec![]);
        }
        let mut g = vec![self[0].inv()];

        let mut d = 1;
        while d < n {
            let next_d = 2 * d;

            let mut f = FPS(vec![Mint::raw(0); next_d]);
            let copy_len = n.min(next_d);
            f[..copy_len].copy_from_slice(&self[..copy_len]);

            let mut g_curr = FPS(vec![Mint::raw(0); next_d]);
            g_curr[..d].copy_from_slice(&g);

            butterfly(&mut f);
            butterfly(&mut g_curr);

            for i in 0..next_d {
                f[i] *= g_curr[i];
            }
            butterfly_inv(&mut f);

            let inv_len = Mint::new(next_d).inv();
            f[..d].fill(Mint::raw(0)); // 上位 d 項は 0 にリセット
            for i in d..next_d {
                f[i] *= -inv_len; // 上位 d 項を反転しつつ INTT の定数倍を適用
            }

            butterfly(&mut f);
            for i in 0..next_d {
                f[i] *= g_curr[i]; // g_curr は既に NTT 済みなので再利用
            }
            butterfly_inv(&mut f);

            g.resize(next_d, Mint::raw(0));
            for i in d..next_d {
                g[i] = f[i] * inv_len; // INTT の定数倍を適用して g に反映
            }
            d = next_d;
        }
        g.truncate(n);
        FPS(g)
    }

    pub fn log(&self) -> FPS {
        let n = self.len();
        assert!(
            n > 0 && self[0] == Mint::raw(1),
            "log is only defined for FPS with a[0] == 1"
        );
        if n == 1 {
            return FPS(vec![Mint::raw(0)]);
        }
        (&self.derivative() * &self.inv()).resized(n).integral()
    }

    pub fn exp(&self) -> FPS {
        let deg = self.len();
        if deg == 0 {
            return FPS(vec![Mint::raw(1)]);
        }
        assert!(
            self[0] == Mint::raw(0),
            "exp is only defined for FPS with f[0] == 0"
        );

        let mut fiv = vec![Mint::raw(0), Mint::raw(1)];
        let mut inplace_integral = |f: &mut Vec<Mint>| {
            let n = f.len();
            let mod_val = Mint::modulus() as usize;
            while fiv.len() <= n {
                let i = fiv.len();
                fiv.push(-fiv[mod_val % i] * Mint::new(mod_val / i));
            }
            f.insert(0, Mint::raw(0));
            for i in 1..=n {
                f[i] *= fiv[i];
            }
        };

        let inplace_diff = |f: &mut Vec<Mint>| {
            if f.is_empty() {
                return;
            }
            f.remove(0);
            let mut coef = Mint::raw(1);
            for i in 0..f.len() {
                f[i] *= coef;
                coef += Mint::raw(1);
            }
        };

        let mut b = vec![Mint::raw(1)];
        if deg > 1 {
            b.push(self[1]);
        }
        let mut c = vec![Mint::raw(1)];
        let mut z1: Vec<Mint>;
        let mut z2 = vec![Mint::raw(1), Mint::raw(1)];

        let mut m = 2;
        while m < deg {
            let mut y = b.clone();
            y.resize(m * 2, Mint::raw(0));
            butterfly(&mut y);

            z1 = z2.clone();
            let mut z = vec![Mint::raw(0); m];
            for i in 0..m {
                z[i] = y[i] * z1[i];
            }
            butterfly_inv(&mut z);
            let inv_m = Mint::new(m).inv();
            for x in &mut z {
                *x *= inv_m;
            }

            z[..m / 2].fill(Mint::raw(0));
            butterfly(&mut z);
            for i in 0..m {
                z[i] *= -z1[i];
            }
            butterfly_inv(&mut z);
            for x in &mut z {
                *x *= inv_m;
            }

            c.extend_from_slice(&z[m / 2..m]);
            z2 = c.clone();
            z2.resize(m * 2, Mint::raw(0));
            butterfly(&mut z2);

            let mut x = FPS(self.0[..deg.min(m)].to_vec());
            inplace_diff(&mut x);
            x.push(Mint::raw(0));
            x.resize(m, Mint::raw(0));
            butterfly(&mut x);
            for i in 0..m {
                x[i] *= y[i];
            }
            butterfly_inv(&mut x);
            x *= inv_m;

            let mut b_diff = b.clone();
            inplace_diff(&mut b_diff);
            for i in 0..b_diff.len() {
                x[i] -= b_diff[i];
            }

            x.resize(m * 2, Mint::raw(0));
            for i in 0..m - 1 {
                x[m + i] = x[i];
                x[i] = Mint::raw(0);
            }
            butterfly(&mut x);
            for i in 0..m * 2 {
                x[i] *= z2[i];
            }
            butterfly_inv(&mut x);
            let inv_2m = Mint::new(m * 2).inv();
            x *= inv_2m;
            x.pop();
            inplace_integral(&mut x);
            for i in m..deg.min(m * 2) {
                x[i] += self[i];
            }
            x[0..m].fill(Mint::raw(0));
            butterfly(&mut x);
            for i in 0..m * 2 {
                x[i] *= y[i];
            }
            butterfly_inv(&mut x);
            x *= inv_2m;

            b.extend_from_slice(&x[m..]);
            m <<= 1;
        }

        b.truncate(deg);
        FPS(b)
    }

    pub fn pow(&self, exp: i64) -> FPS {
        let size = self.len();
        if size == 0 {
            return FPS(vec![]);
        }
        if exp == 0 {
            let mut res = vec![Mint::raw(0); size];
            res[0] = Mint::raw(1);
            return FPS(res);
        }
        if exp < 0 {
            return self.inv().pow(-exp);
        }

        let k = self.iter().position(|&x| x != Mint::raw(0)).unwrap_or(size);
        let shift = k.saturating_mul(exp as usize);
        if shift >= size {
            return FPS(vec![Mint::raw(0); size]);
        }

        let a0 = self[k];
        let inv_a0 = a0.inv();
        let mut h = FPS(self[k..].iter().map(|&x| x * inv_a0).collect());
        h.truncate(size - shift);

        let mut core = (h.log() * Mint::new(exp)).exp();
        core.truncate(size - shift);
        core *= a0.pow(exp as u64);

        let mut res = vec![Mint::raw(0); size];
        res[shift..shift + core.len()].copy_from_slice(&core);
        FPS(res)
    }

    pub fn sqrt(&self) -> Option<FPS> {
        let n = self.len();
        if n == 0 {
            return Some(FPS(vec![]));
        }

        let k = self.iter().position(|&x| x != Mint::raw(0)).unwrap_or(n);
        if k == n {
            return Some(FPS(vec![Mint::raw(0); n]));
        }
        if k % 2 == 1 {
            return None;
        }

        let mut g = FPS(vec![FPS::mod_sqrt(self[k])?]);
        let inv2 = Mint::raw(2).inv();
        let f_shifted = &self[k..];

        let mut d = 1;
        while d < f_shifted.len() {
            d *= 2;
            let mut g_ext = g.clone();
            g_ext.resize(d, Mint::raw(0));

            let f_trunc = FPS(f_shifted[..d.min(f_shifted.len())].to_vec());
            let mut q = &f_trunc * &g_ext.inv();
            q.truncate(d);

            g = (&g_ext + &q) * inv2;
        }

        let mut res = vec![Mint::raw(0); n];
        let shift = k / 2;
        let copy_len = g.len().min(n - shift);
        res[shift..shift + copy_len].copy_from_slice(&g[..copy_len]);

        Some(FPS(res))
    }

    pub fn composition(&self, g: &FPS) -> FPS {
        let deg = self.len().max(g.len());
        if self.is_empty() {
            return FPS(vec![]);
        }
        if g.is_empty() {
            let mut res = vec![Mint::raw(0); deg];
            res[0] = self[0];
            return FPS(res);
        }

        let mut f_vec = self.0.clone();
        let mut g_vec = g.0.clone();
        f_vec.resize(deg, Mint::raw(0));
        g_vec.resize(deg, Mint::raw(0));

        let n = deg - 1;
        let mut h = 1;
        while h < n + 1 {
            h *= 2;
        }
        let k = 1;

        let mut q = vec![Mint::raw(0); h * k];
        for i in 0..=n {
            q[i] = -g_vec[i];
        }

        fn rec(q: Vec<Mint>, n: usize, h: usize, k: usize, f_vec: &[Mint]) -> Vec<Mint> {
            if n == 0 {
                let mut t = q[..k].to_vec();
                t.push(Mint::raw(1));

                let t_rev_inv_rev = FPS::new(t.clone()).rev().inv().rev();

                let u = (FPS::new(f_vec.to_vec()) * t_rev_inv_rev).0;

                let mut p = vec![Mint::raw(0); h * k];
                for i in 0..f_vec.len() {
                    if i + k < u.len() {
                        p[k - i - 1] = u[i + k];
                    }
                }
                return p;
            }

            let mut nq = vec![Mint::raw(0); h * k * 4];
            let mut nr = vec![Mint::raw(0); h * k * 2];
            for i in 0..k {
                nq[i * h * 2..i * h * 2 + n + 1].copy_from_slice(&q[i * h..i * h + n + 1]);
            }
            nq[h * k * 2] += Mint::raw(1);

            butterfly(&mut nq);

            for i in (0..h * k * 4).step_by(2) {
                nq.swap(i, i + 1);
            }

            for i in 0..h * k * 2 {
                nr[i] = nq[i * 2] * nq[i * 2 + 1];
            }

            butterfly_inv(&mut nr);
            let inv_len = Mint::new(h * k * 2).inv();
            for x in &mut nr {
                *x *= inv_len;
            }
            nr[0] -= Mint::raw(1);

            let mut next_q = vec![Mint::raw(0); (h / 2) * (k * 2)];
            for i in 0..k * 2 {
                for j in 0..=n / 2 {
                    next_q[i * (h / 2) + j] = nr[i * h + j];
                }
            }

            let p_res = rec(next_q, n / 2, h / 2, k * 2, f_vec);

            let mut np = FPS(vec![Mint::raw(0); h * k * 4]);
            for i in 0..k * 2 {
                for j in 0..=n / 2 {
                    np[i * h * 2 + j * 2 + n % 2] = p_res[i * (h / 2) + j];
                }
            }

            butterfly(&mut np);

            let mut step = 1;
            while step < h * k * 4 {
                nq[step..step * 2].reverse();
                step <<= 1;
            }

            for i in 0..h * k * 4 {
                np[i] *= nq[i];
            }

            butterfly_inv(&mut np);
            np *= Mint::new(h * k * 4).inv();
            let mut next_p = vec![Mint::raw(0); h * k];
            for i in 0..k {
                next_p[i * h..i * h + n + 1].copy_from_slice(&np[i * h * 2..i * h * 2 + n + 1]);
            }

            next_p
        }

        FPS(rec(q, n, h, k, &f_vec)).resized(n + 1).rev()
    }

    fn shift_right(&self, k: usize) -> FPS {
        if self.len() <= k {
            FPS(vec![])
        } else {
            FPS(self[k..].to_vec())
        }
    }

    fn half_gcd(a: &FPS, b: &FPS) -> PolyMatrix {
        let a = a.clone().trim_trailing_zeros();
        let b = b.clone().trim_trailing_zeros();

        let d = a.len() as isize - 1;
        let d2 = b.len() as isize - 1;
        if d < 0 || d2 < 0 || d < d2 {
            return PolyMatrix::identity();
        }
        let m = ((d + 1) / 2) as usize;
        if m == 0 || d2 < m as isize {
            return PolyMatrix::identity();
        }

        let mut m1 = FPS::half_gcd(&a.shift_right(m), &b.shift_right(m));
        let (mut a2, mut b2) = m1.apply(&a, &b);
        if b2.is_empty() {
            return m1;
        }

        let q = &a2 / &b2;
        let r = a2 - &q * &b2;
        let q_neg = q.clone() * -Mint::raw(1);
        let q_mat = PolyMatrix {
            dat: [
                [FPS(vec![]), FPS(vec![Mint::raw(1)])],
                [FPS(vec![Mint::raw(1)]), q_neg],
            ],
        };
        m1 = q_mat.mul(&m1);
        a2 = b2;
        b2 = r;

        let k = m as isize - (a.len() as isize - a2.len() as isize);
        if b2.len() as isize - 1 < k {
            return m1;
        }

        let m2 = FPS::half_gcd(&a2.shift_right(k as usize), &b2.shift_right(k as usize));
        m2.mul(&m1)
    }

    // Returns (g, x, y) such that g = gcd(a, b) and ax + by = g
    pub fn extgcd(a: &FPS, b: &FPS) -> (FPS, FPS, FPS) {
        let mut a = a.clone().trim_trailing_zeros();
        let mut b = b.clone().trim_trailing_zeros();
        let mut swapped = false;
        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
            swapped = true;
        }
        let mut mat = PolyMatrix::identity();
        while !b.is_empty() {
            let m = FPS::half_gcd(&a, &b);
            mat = m.mul(&mat);
            let (a2, b2) = m.apply(&a, &b);
            if b2.is_empty() {
                a = a2;
                break;
            }
            let q = &a2 / &b2;
            let r = &a2 - &(&q * &b2);
            let q_neg = q.clone() * -Mint::raw(1);
            let q_mat = PolyMatrix {
                dat: [
                    [FPS(vec![]), FPS(vec![Mint::raw(1)])],
                    [FPS(vec![Mint::raw(1)]), q_neg],
                ],
            };
            mat = q_mat.mul(&mat);
            a = b2;
            b = r;
        }
        if !a.is_empty() {
            let inv = a.last().unwrap().inv();
            a *= inv;
            for j in 0..2 {
                mat.dat[0][j] *= inv;
            }
        }
        if swapped {
            (a, mat.dat[0][1].clone(), mat.dat[0][0].clone())
        } else {
            (a, mat.dat[0][0].clone(), mat.dat[0][1].clone())
        }
    }

    pub fn inv_mod(&self, b: &FPS) -> Option<FPS> {
        let (g, x, _) = FPS::extgcd(self, b);
        if g.len() == 1 && g[0] != Mint::raw(0) {
            Some(&(x * g[0].inv()) % b)
        } else {
            None
        }
    }

    fn build_subproduct_tree(points: &[Mint]) -> Vec<FPS> {
        let m = points.len();
        if m == 0 {
            return vec![];
        }
        let n = m.next_power_of_two();
        let mut tree = vec![FPS(vec![]); 2 * n];
        for i in 0..n {
            let w = if i < m { -points[i] } else { Mint::raw(0) };
            tree[n + i] = FPS(vec![w, Mint::raw(1)]);
        }
        for i in (1..n).rev() {
            tree[i] = &tree[i * 2] * &tree[i * 2 + 1];
        }
        tree
    }

    pub fn multipoint_evaluation(&self, points: &[Mint]) -> Vec<Mint> {
        let m = points.len();
        if m == 0 {
            return vec![];
        }
        let n = m.next_power_of_two();
        let tree = FPS::build_subproduct_tree(points);
        let f_len = self.len();
        if f_len == 0 {
            return vec![Mint::raw(0); m];
        }

        let mut c = vec![FPS(vec![]); 2 * n];

        let mut v = &tree[1].clone().rev().resized(f_len).inv().rev() * self;
        v.0.drain(0..f_len - 1);
        c[1] = v.resized(n).rev();

        for i in 1..n {
            let d = c[i].len();
            let inv_d = Mint::new(d).inv();
            butterfly(&mut c[i]);
            let mut add = tree[i * 2].clone().resized(d / 2 + 1).rev().resized(d);
            butterfly(&mut add);
            c[i * 2 + 1].resize(d, Mint::raw(0));
            for j in 0..d {
                c[i * 2 + 1][j] = c[i][j] * add[j];
            }
            butterfly_inv(&mut c[i * 2 + 1]);
            for j in 0..d {
                c[i * 2 + 1][j] *= inv_d;
            }
            c[i * 2 + 1].drain(..d / 2);
            add = tree[i * 2 + 1].clone().resized(d / 2 + 1).rev().resized(d);
            butterfly(&mut add);
            c[i * 2].resize(d, Mint::raw(0));
            for j in 0..d {
                c[i * 2][j] = c[i][j] * add[j];
            }
            butterfly_inv(&mut c[i * 2]);
            for j in 0..c[i * 2].len() {
                c[i * 2][j] *= inv_d;
            }
            c[i * 2].drain(..d / 2);
        }
        c[n..n + m].iter().map(|poly| poly[0]).collect()
    }

    pub fn multipoint_evaluation_on_geometric_seq(&self, w: Mint, n: usize, a: Mint) -> Vec<Mint> {
        if self.is_empty() || n == 0 {
            return vec![Mint::raw(0); n];
        }

        let m = self.len();
        let mut f = self.0.clone();

        if a != Mint::raw(1) {
            let mut x = Mint::raw(1);
            for c in &mut f {
                *c *= x;
                x *= a;
            }
        }

        if w == Mint::raw(0) {
            let mut result = vec![f[0]; n];
            result[0] = f.into_iter().fold(Mint::raw(0), |acc, v| acc + v);
            return result;
        }

        let build_powers = |base: Mint, len: usize| {
            let mut powers = Vec::with_capacity(len);
            let mut current = Mint::raw(1);
            let mut step = Mint::raw(1);
            for _ in 0..len {
                powers.push(current);
                current *= step;
                step *= base;
            }
            powers
        };

        let wc = build_powers(w, n + m - 1);
        let iwc = build_powers(w.inv(), m.max(n));

        for (c, &iwc_val) in f.iter_mut().zip(&iwc) {
            *c *= iwc_val;
        }

        f.reverse();
        let g = convolution(&f, &wc);

        g.into_iter()
            .skip(m - 1)
            .chain(std::iter::repeat(Mint::raw(0)))
            .take(n)
            .zip(&iwc)
            .map(|(g_val, &iwc_val)| g_val * iwc_val)
            .collect()
    }

    pub fn interpolate(xs: &[Mint], ys: &[Mint]) -> Self {
        let m = xs.len();
        let n = m.next_power_of_two();
        let mut tree = vec![FPS(vec![Mint::raw(1)]); n * 2];
        for i in 0..m {
            tree[n + i] = FPS(vec![-xs[i], Mint::raw(1)]);
        }
        for i in (1..n).rev() {
            tree[i] = &tree[i * 2] * &tree[i * 2 + 1];
        }
        let f = tree[1].derivative();
        let mut g = vec![FPS(vec![]); n * 2];
        g[1] = &f % &tree[1];
        for i in 2..n + m {
            g[i] = &g[i / 2] % &tree[i];
        }
        for i in 0..m {
            g[n + i] = FPS(vec![ys[i] / g[n + i][0]]);
        }
        for i in (0..n).rev() {
            g[i] = &tree[i * 2 + 1] * &g[i * 2] + &tree[i * 2] * &g[i * 2 + 1];
        }
        g[1].clone()
    }

    pub fn interpolate_on_geometric_seq(v: &[Mint], a: Mint, q: Mint) -> Self {
        let n = v.len();
        if n == 0 {
            return FPS(vec![]);
        }
        if n == 1 {
            return FPS(vec![v[0]]);
        }

        let zero = Mint::raw(0);
        let one = Mint::raw(1);

        let mut q_pow = vec![one; n + 1];
        let mut s = vec![one; n + 1];
        for i in 1..=n {
            q_pow[i] = q_pow[i - 1] * q;
            s[i] = s[i - 1] * (one - q_pow[i]);
        }

        let mut s_inv = vec![zero; n];
        s_inv[n - 1] = s[n - 1].inv();
        for i in (1..n).rev() {
            s_inv[i - 1] = s_inv[i] * (one - q_pow[i]);
        }

        let mut btm = vec![zero; n];
        let mut btm_term = one;
        for i in 0..n {
            btm[i] = FPS::sign(i) * btm_term * s[i] * s[n - 1 - i];
            if i < n - 1 {
                btm_term *= q_pow[n - i - 2];
            }
        }

        let mut btm_inv = vec![zero; n];
        let mut prefix = vec![one; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] * btm[i];
        }
        let mut inv_acc = prefix[n].inv();
        for i in (0..n).rev() {
            btm_inv[i] = inv_acc * prefix[i];
            inv_acc *= btm[i];
        }

        let mut w = vec![zero; n];
        for i in 0..n {
            w[i] = v[i] * btm_inv[i];
        }

        let mut f1 = vec![one; n + 1];
        let mut q_term = one;
        for i in 1..=n {
            q_term *= q_pow[i - 1];
            if i == n {
                f1[i] = FPS::sign(i) * q_term;
            } else {
                f1[i] = FPS::sign(i) * q_term * s[n] * s_inv[i] * s_inv[n - i];
            }
        }

        let f2 = FPS(w).multipoint_evaluation_on_geometric_seq(q, n, one);
        let mut f = FPS(convolution(&f1, &f2));
        f.truncate(n);
        f.reverse();

        let a_inv = a.inv();
        let mut p = one;
        for i in 1..n {
            p *= a_inv;
            f[i] *= p;
        }

        f
    }

    pub fn taylor_shift(&self, c: Mint) -> Self {
        let n = self.len();
        let (fact, inv_fact) = FPS::factorials(n * 2);
        let a = self
            .iter()
            .zip(fact)
            .map(|(&coef, fct)| coef * fct)
            .collect::<Vec<_>>();

        let mut b = FPS(vec![Mint::raw(0); n]);
        let mut pow_c = Mint::raw(1);
        for i in 0..n {
            b[i] = pow_c * inv_fact[i];
            pow_c *= c;
        }
        let conv = convolution(&a, &b.rev());
        let mut res = vec![Mint::raw(0); n];
        for i in 0..n {
            res[i] = conv[n - 1 + i] * inv_fact[i];
        }
        FPS(res)
    }

    fn mod_pow(&self, e: usize, mod_f: &Self) -> Self {
        if e == 0 {
            return FPS(vec![Mint::raw(1)]);
        }
        if e == 1 {
            return self.clone();
        }
        let mut F = self.mod_pow(e / 2, mod_f);
        F = &F * &F;
        F = &F % mod_f;
        if e % 2 == 1 {
            F = &F * self;
            F = &F % mod_f;
        }
        F
    }
}

struct SimpleRng {
    state: u64,
}
impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }
    /// [low, high) の範囲で乱数を返す
    fn gen_range(&mut self, low: usize, high: usize) -> usize {
        low + (self.next() as usize % (high - low))
    }
}

fn find_polynomial_roots(f: &FPS) -> Vec<Mint> {
    let p = Mint::modulus() as usize;
    fn rec(res: &mut Vec<Mint>, f: &FPS, rng: &mut SimpleRng) {
        if f.len() == 1 {
            return;
        }
        if f.len() == 2 {
            res.push(-f[0] / f[1]);
            return;
        }

        let p = Mint::modulus() as usize;

        let g = FPS(vec![Mint::new(rng.gen_range(0, p)), Mint::raw(1)]);
        let mut h = g.mod_pow((p - 1) / 2, f).trim_trailing_zeros();
        if h.is_empty() {
            return rec(res, f, rng);
        }
        h[0] -= Mint::raw(1);
        let f1 = FPS::extgcd(f, &h).0.trim_trailing_zeros();
        let f2 = (f / &f1).trim_trailing_zeros();
        rec(res, &f1, rng);
        rec(res, &f2, rng);
    }

    let x = FPS(vec![Mint::raw(0), Mint::raw(1)]);
    let mut g = x.mod_pow(p, f) - x;
    g = FPS::extgcd(f, &g).0.trim_trailing_zeros();

    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let mut rng = SimpleRng::new(seed ^ 0x9E37_79B9_7F4A_7C15);

    let mut res = Vec::new();

    rec(&mut res, &g, &mut rng);
    res
}

#[derive(Clone)]
pub struct PolyMatrix {
    pub dat: [[FPS; 2]; 2],
}

impl PolyMatrix {
    pub fn identity() -> Self {
        Self {
            dat: [
                [FPS(vec![Mint::raw(1)]), FPS(vec![])],
                [FPS(vec![]), FPS(vec![Mint::raw(1)])],
            ],
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut res = [[FPS(vec![]), FPS(vec![])], [FPS(vec![]), FPS(vec![])]];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let prod = &self.dat[i][k] * &other.dat[k][j];
                    res[i][j] = (&res[i][j] + &prod).trim_trailing_zeros();
                }
            }
        }
        Self { dat: res }
    }

    pub fn apply(&self, a: &FPS, b: &FPS) -> (FPS, FPS) {
        let mut res_a = &(&self.dat[0][0] * a) + &(&self.dat[0][1] * b);
        let mut res_b = &(&self.dat[1][0] * a) + &(&self.dat[1][1] * b);
        (res_a.trim_trailing_zeros(), res_b.trim_trailing_zeros())
    }
}

// 以下convolution
use std::{
    cmp,
    convert::{TryFrom, TryInto as _},
    fmt,
};

pub fn convolution<M>(a: &[StaticModInt<M>], b: &[StaticModInt<M>]) -> Vec<StaticModInt<M>>
where
    M: Modulus,
{
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let (n, m) = (a.len(), b.len());

    if cmp::min(n, m) <= 60 {
        let (n, m, a, b) = if n < m { (m, n, b, a) } else { (n, m, a, b) };
        let mut ans = vec![StaticModInt::new(0); n + m - 1];
        for i in 0..n {
            for j in 0..m {
                ans[i + j] += a[i] * b[j];
            }
        }
        return ans;
    }

    let (mut a, mut b) = (a.to_owned(), b.to_owned());
    let z = 1 << ceil_pow2((n + m - 1) as _);
    a.resize(z, StaticModInt::raw(0));
    butterfly(&mut a);
    b.resize(z, StaticModInt::raw(0));
    butterfly(&mut b);
    for (a, b) in a.iter_mut().zip(&b) {
        *a *= b;
    }
    butterfly_inv(&mut a);
    a.resize(n + m - 1, StaticModInt::raw(0));
    let iz = StaticModInt::new(z).inv();
    for a in &mut a {
        *a *= iz;
    }
    a
}

pub fn convolution_raw<T, M>(a: &[T], b: &[T]) -> Vec<T>
where
    T: RemEuclidU32 + TryFrom<u32> + Clone,
    T::Error: fmt::Debug,
    M: Modulus,
{
    let a = a.iter().cloned().map(Into::into).collect::<Vec<_>>();
    let b = b.iter().cloned().map(Into::into).collect::<Vec<_>>();
    convolution::<M>(&a, &b)
        .into_iter()
        .map(|z| {
            z.val()
                .try_into()
                .expect("the numeric type is smaller than the modulus")
        })
        .collect()
}

fn butterfly<M: Modulus>(a: &mut [StaticModInt<M>]) {
    let n = a.len();
    let h = ceil_pow2(n as u32);

    M::butterfly_cache().with(|cache| {
        let mut cache = cache.borrow_mut();
        let ButterflyCache { sum_e, .. } = cache.get_or_insert_with(prepare);
        for ph in 1..=h {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut now = StaticModInt::<M>::new(1);
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p] * now;
                    a[i + offset] = l + r;
                    a[i + offset + p] = l - r;
                }
                now *= sum_e[(!s).trailing_zeros() as usize];
            }
        }
    });
}

fn butterfly_inv<M: Modulus>(a: &mut [StaticModInt<M>]) {
    let n = a.len();
    let h = ceil_pow2(n as u32);

    M::butterfly_cache().with(|cache| {
        let mut cache = cache.borrow_mut();
        let ButterflyCache { sum_ie, .. } = cache.get_or_insert_with(prepare);
        for ph in (1..=h).rev() {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut inow = StaticModInt::<M>::new(1);
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p];
                    a[i + offset] = l + r;
                    a[i + offset + p] = StaticModInt::new(M::VALUE + l.val() - r.val()) * inow;
                }
                inow *= sum_ie[(!s).trailing_zeros() as usize];
            }
        }
    });
}

fn prepare<M: Modulus>() -> ButterflyCache<M> {
    let g = StaticModInt::<M>::raw(primitive_root(M::VALUE as i32) as u32);
    let mut es = [StaticModInt::<M>::raw(0); 30]; // es[i]^(2^(2+i)) == 1
    let mut ies = [StaticModInt::<M>::raw(0); 30];
    let cnt2 = (M::VALUE - 1).trailing_zeros() as usize;
    let mut e = g.pow(((M::VALUE - 1) >> cnt2).into());
    let mut ie = e.inv();
    for i in (2..=cnt2).rev() {
        es[i - 2] = e;
        ies[i - 2] = ie;
        e *= e;
        ie *= ie;
    }
    let sum_e = es
        .iter()
        .scan(StaticModInt::new(1), |acc, e| {
            *acc *= e;
            Some(*acc)
        })
        .collect();
    let sum_ie = ies
        .iter()
        .scan(StaticModInt::new(1), |acc, ie| {
            *acc *= ie;
            Some(*acc)
        })
        .collect();
    ButterflyCache { sum_e, sum_ie }
}

use std::{
    cell::RefCell,
    convert::Infallible,
    hash::{Hash, Hasher},
    iter::{Product, Sum},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
    thread::LocalKey,
};

pub type ModInt998244353 = StaticModInt<Mod998244353>;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct StaticModInt<M> {
    val: u32,
    phantom: PhantomData<fn() -> M>,
}

impl<M: Modulus> StaticModInt<M> {
    #[inline(always)]
    pub fn modulus() -> u32 {
        M::VALUE
    }

    #[inline]
    pub fn new<T: RemEuclidU32>(val: T) -> Self {
        Self::raw(val.rem_euclid_u32(M::VALUE))
    }

    #[inline]
    pub fn raw(val: u32) -> Self {
        Self {
            val,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn val(self) -> u32 {
        self.val
    }

    #[inline]
    pub fn pow(self, n: u64) -> Self {
        <Self as ModIntBase>::pow(self, n)
    }

    #[inline]
    pub fn inv(self) -> Self {
        if M::HINT_VALUE_IS_PRIME {
            if self.val() == 0 {
                panic!("attempt to divide by zero");
            }
            debug_assert!(
                is_prime(M::VALUE.try_into().unwrap()),
                "{} is not a prime number",
                M::VALUE,
            );
            self.pow((M::VALUE - 2).into())
        } else {
            Self::inv_for_non_prime_modulus(self)
        }
    }
}

impl<M: Modulus> ModIntBase for StaticModInt<M> {
    #[inline(always)]
    fn modulus() -> u32 {
        Self::modulus()
    }

    #[inline]
    fn raw(val: u32) -> Self {
        Self::raw(val)
    }

    #[inline]
    fn val(self) -> u32 {
        self.val()
    }

    #[inline]
    fn inv(self) -> Self {
        self.inv()
    }
}

pub trait Modulus: 'static + Copy + Eq {
    const VALUE: u32;
    const HINT_VALUE_IS_PRIME: bool;

    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>>;
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Mod998244353 {}

impl Modulus for Mod998244353 {
    const VALUE: u32 = 998_244_353;
    const HINT_VALUE_IS_PRIME: bool = true;

    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
        thread_local! {
            static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod998244353>>> = RefCell::default();
        }
        &BUTTERFLY_CACHE
    }
}
pub struct ButterflyCache<M> {
    pub(crate) sum_e: Vec<StaticModInt<M>>,
    pub(crate) sum_ie: Vec<StaticModInt<M>>,
}
pub trait ModIntBase:
    Default
    + FromStr
    + From<i8>
    + From<i16>
    + From<i32>
    + From<i64>
    + From<i128>
    + From<isize>
    + From<u8>
    + From<u16>
    + From<u32>
    + From<u64>
    + From<u128>
    + From<usize>
    + Copy
    + Eq
    + Hash
    + fmt::Display
    + fmt::Debug
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
    fn modulus() -> u32;

    fn raw(val: u32) -> Self;

    fn val(self) -> u32;

    fn inv(self) -> Self;

    #[inline]
    fn new<T: RemEuclidU32>(val: T) -> Self {
        Self::raw(val.rem_euclid_u32(Self::modulus()))
    }

    #[inline]
    fn pow(self, mut n: u64) -> Self {
        let mut x = self;
        let mut r = Self::raw(u32::from(Self::modulus() > 1));
        while n > 0 {
            if n & 1 == 1 {
                r *= x;
            }
            x *= x;
            n >>= 1;
        }
        r
    }
}

pub trait RemEuclidU32 {
    fn rem_euclid_u32(self, modulus: u32) -> u32;
}

macro_rules! impl_rem_euclid_u32_for_small_signed {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    (self as i64).rem_euclid(i64::from(modulus)) as _
                }
            }
        )*
    }
}

impl_rem_euclid_u32_for_small_signed!(i8, i16, i32, i64, isize);

impl RemEuclidU32 for i128 {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        self.rem_euclid(i128::from(modulus)) as _
    }
}

macro_rules! impl_rem_euclid_u32_for_small_unsigned {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    self as u32 % modulus
                }
            }
        )*
    }
}

macro_rules! impl_rem_euclid_u32_for_large_unsigned {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    (self % (modulus as $ty)) as _
                }
            }
        )*
    }
}

impl_rem_euclid_u32_for_small_unsigned!(u8, u16, u32);
impl_rem_euclid_u32_for_large_unsigned!(u64, u128);

#[cfg(target_pointer_width = "32")]
impl_rem_euclid_u32_for_small_unsigned!(usize);

#[cfg(target_pointer_width = "64")]
impl_rem_euclid_u32_for_large_unsigned!(usize);

trait InternalImplementations: ModIntBase {
    #[inline]
    fn inv_for_non_prime_modulus(this: Self) -> Self {
        let (gcd, x) = inv_gcd(this.val().into(), Self::modulus().into());
        if gcd != 1 {
            panic!("the multiplicative inverse does not exist");
        }
        Self::new(x)
    }

    #[inline]
    fn default_impl() -> Self {
        Self::raw(0)
    }

    #[inline]
    fn from_str_impl(s: &str) -> Result<Self, Infallible> {
        Ok(s.parse::<i64>()
            .map(Self::new)
            .unwrap_or_else(|_| todo!("parsing as an arbitrary precision integer?")))
    }

    #[inline]
    fn hash_impl(this: &Self, state: &mut impl Hasher) {
        this.val().hash(state)
    }

    #[inline]
    fn display_impl(this: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&this.val(), f)
    }

    #[inline]
    fn debug_impl(this: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&this.val(), f)
    }

    #[inline]
    fn neg_impl(this: Self) -> Self {
        Self::sub_impl(Self::raw(0), this)
    }

    #[inline]
    fn add_impl(lhs: Self, rhs: Self) -> Self {
        let modulus = Self::modulus();
        let mut val = lhs.val() + rhs.val();
        if val >= modulus {
            val -= modulus;
        }
        Self::raw(val)
    }

    #[inline]
    fn sub_impl(lhs: Self, rhs: Self) -> Self {
        let modulus = Self::modulus();
        let mut val = lhs.val().wrapping_sub(rhs.val());
        if val >= modulus {
            val = val.wrapping_add(modulus)
        }
        Self::raw(val)
    }

    fn mul_impl(lhs: Self, rhs: Self) -> Self;

    #[inline]
    fn div_impl(lhs: Self, rhs: Self) -> Self {
        Self::mul_impl(lhs, rhs.inv())
    }
}

impl<M: Modulus> InternalImplementations for StaticModInt<M> {
    #[inline]
    fn mul_impl(lhs: Self, rhs: Self) -> Self {
        Self::raw((u64::from(lhs.val()) * u64::from(rhs.val()) % u64::from(M::VALUE)) as u32)
    }
}

macro_rules! impl_basic_traits {
    () => {};
    (impl <$generic_param:ident : $generic_param_bound:tt> _ for $self:ty; $($rest:tt)*) => {
        impl <$generic_param: $generic_param_bound> Default for $self {
            #[inline]
            fn default() -> Self {
                Self::default_impl()
            }
        }

        impl <$generic_param: $generic_param_bound> FromStr for $self {
            type Err = Infallible;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Infallible> {
                Self::from_str_impl(s)
            }
        }

        impl<$generic_param: $generic_param_bound, V: RemEuclidU32> From<V> for $self {
            #[inline]
            fn from(from: V) -> Self {
                Self::new(from)
            }
        }

        #[allow(clippy::derived_hash_with_manual_eq)]
        impl<$generic_param: $generic_param_bound> Hash for $self {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                Self::hash_impl(self, state)
            }
        }

        impl<$generic_param: $generic_param_bound> fmt::Display for $self {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Self::display_impl(self, f)
            }
        }

        impl<$generic_param: $generic_param_bound> fmt::Debug for $self {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Self::debug_impl(self, f)
            }
        }

        impl<$generic_param: $generic_param_bound> Neg for $self {
            type Output = $self;

            #[inline]
            fn neg(self) -> $self {
                Self::neg_impl(self)
            }
        }

        impl<$generic_param: $generic_param_bound> Neg for &'_ $self {
            type Output = $self;

            #[inline]
            fn neg(self) -> $self {
                <$self>::neg_impl(*self)
            }
        }

        impl_basic_traits!($($rest)*);
    };
}

impl_basic_traits! {
    impl <M: Modulus> _ for StaticModInt<M> ;
}

macro_rules! impl_bin_ops {
    () => {};
    (for<$($generic_param:ident : $generic_param_bound:tt),*> <$lhs_ty:ty> ~ <$rhs_ty:ty> -> $output:ty { { $lhs_body:expr } ~ { $rhs_body:expr } } $($rest:tt)*) => {
        impl <$($generic_param: $generic_param_bound),*> Add<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs_ty) -> $output {
                <$output>::add_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$($generic_param: $generic_param_bound),*> Sub<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: $rhs_ty) -> $output {
                <$output>::sub_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$($generic_param: $generic_param_bound),*> Mul<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: $rhs_ty) -> $output {
                <$output>::mul_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$($generic_param: $generic_param_bound),*> Div<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn div(self, rhs: $rhs_ty) -> $output {
                <$output>::div_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl_bin_ops!($($rest)*);
    };
}

macro_rules! impl_assign_ops {
    () => {};
    (for<$($generic_param:ident : $generic_param_bound:tt),*> <$lhs_ty:ty> ~= <$rhs_ty:ty> { _ ~= { $rhs_body:expr } } $($rest:tt)*) => {
        impl <$($generic_param: $generic_param_bound),*> AddAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs_ty) {
                *self = *self + apply($rhs_body, rhs);
            }
        }

        impl <$($generic_param: $generic_param_bound),*> SubAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn sub_assign(&mut self, rhs: $rhs_ty) {
                *self = *self - apply($rhs_body, rhs);
            }
        }

        impl <$($generic_param: $generic_param_bound),*> MulAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn mul_assign(&mut self, rhs: $rhs_ty) {
                *self = *self * apply($rhs_body, rhs);
            }
        }

        impl <$($generic_param: $generic_param_bound),*> DivAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn div_assign(&mut self, rhs: $rhs_ty) {
                *self = *self / apply($rhs_body, rhs);
            }
        }

        impl_assign_ops!($($rest)*);
    };
}

#[inline]
fn apply<F: FnOnce(X) -> O, X, O>(f: F, x: X) -> O {
    f(x)
}

impl_bin_ops! {
    for<M: Modulus> <StaticModInt<M>     > ~ <StaticModInt<M>     > -> StaticModInt<M>  { { |x| x  } ~ { |x| x  } }
    for<M: Modulus> <StaticModInt<M>     > ~ <&'_ StaticModInt<M> > -> StaticModInt<M>  { { |x| x  } ~ { |&x| x } }
    for<M: Modulus> <&'_ StaticModInt<M> > ~ <StaticModInt<M>     > -> StaticModInt<M>  { { |&x| x } ~ { |x| x  } }
    for<M: Modulus> <&'_ StaticModInt<M> > ~ <&'_ StaticModInt<M> > -> StaticModInt<M>  { { |&x| x } ~ { |&x| x } }

    for<M: Modulus, T: RemEuclidU32> <StaticModInt<M>     > ~ <T> -> StaticModInt<M>  { { |x| x  } ~ { StaticModInt::<M>::new } }
}

impl_assign_ops! {
    for<M: Modulus> <StaticModInt<M> > ~= <StaticModInt<M>     > { _ ~= { |x| x  } }
    for<M: Modulus> <StaticModInt<M> > ~= <&'_ StaticModInt<M> > { _ ~= { |&x| x } }

    for<M: Modulus, T: RemEuclidU32> <StaticModInt<M> > ~= <T> { _ ~= { StaticModInt::<M>::new } }
}

macro_rules! impl_folding {
    () => {};
    (impl<$generic_param:ident : $generic_param_bound:tt> $trait:ident<_> for $self:ty { fn $method:ident(_) -> _ { _($unit:expr, $op:expr) } } $($rest:tt)*) => {
        impl<$generic_param: $generic_param_bound> $trait<Self> for $self {
            #[inline]
            fn $method<S>(iter: S) -> Self
            where
                S: Iterator<Item = Self>,
            {
                iter.fold($unit, $op)
            }
        }

        impl<'a, $generic_param: $generic_param_bound> $trait<&'a Self> for $self {
            #[inline]
            fn $method<S>(iter: S) -> Self
            where
                S: Iterator<Item = &'a Self>,
            {
                iter.fold($unit, $op)
            }
        }

        impl_folding!($($rest)*);
    };
}

impl_folding! {
    impl<M: Modulus> Sum<_>     for StaticModInt<M>  { fn sum(_)     -> _ { _(Self::raw(0), Add::add) } }
    impl<M: Modulus> Product<_> for StaticModInt<M>  { fn product(_) -> _ { _(Self::raw(u32::from(Self::modulus() > 1)), Mul::mul) } }
}

pub(crate) fn ceil_pow2(n: u32) -> u32 {
    32 - n.saturating_sub(1).leading_zeros()
}

pub(crate) fn safe_mod(mut x: i64, m: i64) -> i64 {
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}

pub(crate) fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
    let a = safe_mod(a, b);
    if a == 0 {
        return (b, 0);
    }
    let mut s = b;
    let mut t = a;
    let mut m0 = 0;
    let mut m1 = 1;

    while t != 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u; // |m1 * u| <= |m1| * s <= b

        // [3]:
        // (s - t * u) * |m1| + t * |m0 - m1 * u|
        // <= s * |m1| - t * u * |m1| + t * (|m0| + |m1| * u)
        // = s * |m1| + t * |m0| <= b

        std::mem::swap(&mut s, &mut t);
        std::mem::swap(&mut m0, &mut m1);
    }
    // by [3]: |m0| <= b/g
    // by g != b: |m0| < b/g
    if m0 < 0 {
        m0 += b / s;
    }
    (s, m0)
}

pub(crate) fn primitive_root(m: i32) -> i32 {
    match m {
        2 => return 1,
        167_772_161 => return 3,
        469_762_049 => return 3,
        754_974_721 => return 11,
        998_244_353 => return 3,
        _ => {}
    }

    let mut divs = [0; 20];
    divs[0] = 2;
    let mut cnt = 1;
    let mut x = (m - 1) / 2;
    while x % 2 == 0 {
        x /= 2;
    }
    for i in (3..i32::MAX).step_by(2) {
        if i as i64 * i as i64 > x as i64 {
            break;
        }
        if x % i == 0 {
            divs[cnt] = i;
            cnt += 1;
            while x % i == 0 {
                x /= i;
            }
        }
    }
    if x > 1 {
        divs[cnt] = x;
        cnt += 1;
    }
    let mut g = 2;
    loop {
        if (0..cnt).all(|i| pow_mod(g, ((m - 1) / divs[i]) as i64, m) != 1) {
            break g as i32;
        }
        g += 1;
    }
}

pub(crate) fn pow_mod(x: i64, mut n: i64, m: i32) -> i64 {
    if m == 1 {
        return 0;
    }
    let _m = m as u32;
    let mut r: u64 = 1;
    let mut y: u64 = safe_mod(x, m as i64) as u64;
    while n != 0 {
        if (n & 1) > 0 {
            r = (r * y) % (_m as u64);
        }
        y = (y * y) % (_m as u64);
        n >>= 1;
    }
    r as i64
}

pub(crate) fn is_prime(n: i32) -> bool {
    let n = n as i64;
    match n {
        _ if n <= 1 => return false,
        2 | 7 | 61 => return true,
        _ if n % 2 == 0 => return false,
        _ => {}
    }
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }
    for &a in &[2, 7, 61] {
        let mut t = d;
        let mut y = pow_mod(a, t, n as i32);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = y * y % n;
            t <<= 1;
        }
        if y != n - 1 && t % 2 == 0 {
            return false;
        }
    }
    true
}
