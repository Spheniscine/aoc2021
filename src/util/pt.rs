use std::ops::*;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pt<const N: usize>(pub [i64; N]);
impl <const N: usize> From<[i64; N]> for Pt<N> {
    fn from(x: [i64; N]) -> Self {
        Self(x)
    }
}

impl <const N: usize> Hash for Pt<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..N {
            state.write_i64(self[i]);
        }
    }
}

impl <const N: usize> Index<usize> for Pt<N> {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl <const N: usize> IndexMut<usize> for Pt<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl <const N: usize> Default for Pt<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl <const N: usize> Add for Pt<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut r = self;
        for i in 0..N {
            r[i] += rhs[i];
        }
        r
    }
}

impl <const N: usize> Sub for Pt<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut r = self;
        for i in 0..N {
            r[i] -= rhs[i];
        }
        r
    }
}

impl <const N: usize> Neg for Pt<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut r = self;
        for i in 0..N {
            r[i] = -r[i];
        }
        r
    }
}

impl <const N: usize> Pt<N> {
    pub fn manhattan_distance(self) -> i64 {
        let mut ans = 0;
        for i in 0..N {
            ans += self[i].abs();
        }
        ans
    }
}