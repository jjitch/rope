#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModInt<const MOD: i64>(i64);

impl<const MOD: i64> ModInt<{ MOD }> {
    /// Constructs new ModInt.
    ///
    /// A given `x` is clamped into `[0, MOD)`.
    pub fn new(x: i64) -> ModInt<{ MOD }> {
        ModInt((x + MOD) % MOD)
    }
    pub const fn m() -> i64 {
        MOD
    }
    pub fn pow(self, p: i64) -> ModInt<{ MOD }> {
        if p == 0 {
            ModInt::new(1)
        } else {
            let s = if p < 0 { self.inv() } else { self };
            let p = if p < 0 { -p } else { p };
            let e = Self::pow(s, p >> 1);
            if p & 1 == 1 {
                e * e * s
            } else {
                e * e
            }
        }
    }
    pub fn inv(self) -> ModInt<{ MOD }> {
        let mut a = self.0;
        let (mut b, mut u, mut v) = (MOD, 1, 0);
        while b > 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        Self::new(u)
    }
}

macro_rules! impl_bin_op {
    ($bin_op_trait: ident, $bin_op_func: ident) => {
        impl<T, const MOD: i64> std::ops::$bin_op_trait<T> for ModInt<{ MOD }>
        where
            T: Into<ModInt<{ MOD }>>,
        {
            type Output = Self;
            fn $bin_op_func(self, rhs: T) -> Self::Output {
                Self::new(self.0.$bin_op_func(rhs.into().0))
            }
        }
    };
}

impl_bin_op!(Add, add);
impl_bin_op!(Sub, sub);
impl_bin_op!(Mul, mul);
impl_bin_op!(Div, div);

macro_rules! impl_assign_op {
    ($assign_trait: ident, $assign_func: ident) => {
        impl<T, const MOD: i64> std::ops::$assign_trait<T> for ModInt<{ MOD }>
        where
            T: Into<ModInt<{ MOD }>>,
        {
            fn $assign_func(&mut self, rhs: T) {
                self.0.$assign_func(rhs.into().0);
            }
        }
    };
}

impl_assign_op!(AddAssign, add_assign);
impl_assign_op!(SubAssign, sub_assign);
impl_assign_op!(MulAssign, mul_assign);
impl_assign_op!(DivAssign, div_assign);

impl<T, const MOD: i64> std::ops::Index<ModInt<{ MOD }>> for Vec<T> {
    type Output = T;
    fn index(&self, index: ModInt<MOD>) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl<T, const MOD: i64> std::ops::IndexMut<ModInt<{ MOD }>> for Vec<T> {
    fn index_mut(&mut self, index: ModInt<MOD>) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}

impl<const MOD: i64> From<i64> for ModInt<{ MOD }> {
    fn from(x: i64) -> Self {
        Self::new(x)
    }
}

impl<const MOD: i64> std::str::FromStr for ModInt<{ MOD }> {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ModInt::new(i64::from_str(s)?))
    }
}

impl<const MOD: i64> std::fmt::Display for ModInt<{ MOD }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const MOD: i64> std::fmt::Debug for ModInt<{ MOD }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} mod {}", self.0, MOD)
    }
}
