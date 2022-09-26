
use std::{ops::{Add, Mul, Sub, Neg}, str::MatchIndices, process::Output};

use num::Num;

#[derive(Clone, Copy, Debug)]
// #[repr(C)]
pub struct Matrix2x2<S: Num> {
    pub a1: S, pub a2: S, 
    pub b1: S, pub b2: S, 
}

impl<S: Num> Matrix2x2<S> {
    pub fn new(a1: S, a2: S, 
               b1: S, b2: S) -> Self {
        Self {a1, a2,
              b1, b2}
    }

    pub fn zero() -> Self {
        Self::new(S::zero(),S::zero(),
                  S::zero(),S::zero(),)
    }
    pub fn one() -> Self {
        Self::new(S::one(), S::one(),
                  S::one(), S::one(),)
    }
    pub fn id() -> Self {
        Self::new(S::one(), S::zero(),
                  S::zero(), S::one(),)
    }
}

impl<T: Num> Add<Self> for Matrix2x2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.a1 + rhs.a1, self.a2 + rhs.a2,
                  self.b1 + rhs.b1, self.b2 + rhs.b2,)
    }
}
impl<T: Num + Copy> Sub<Self> for Matrix2x2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.a1 - rhs.a1, self.a2 - rhs.a2,
                  self.a1 - rhs.a1, self.a2 - rhs.a2,)
    }
}
impl<T: Num + Neg<Output=T>> Neg for Matrix2x2<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.a1, -self.a2,
                  -self.b1, -self.b2,)
    }
}
impl<T: Num + Copy> Mul<Matrix2x2<T>> for Matrix2x2<T> {
    type Output = Matrix2x2<T>;
    fn mul(self, rhs: Matrix2x2<T>) -> Matrix2x2<T> {
        Matrix2x2::new(
            self.a1 * rhs.a1 + self.a2 * rhs.b1, self.a1 * rhs.a2 + self.a2 * rhs.b2,
            self.b1 * rhs.a1 + self.b2 * rhs.b1, self.b1 * rhs.a2 + self.b2 * rhs.b2,)
    }
}


macro_rules! impl_mul_mat2 {
    ($($T:ty),*) => {$(

impl Mul<Matrix2x2<$T>> for $T {
    type Output = Matrix2x2<$T>;
    fn mul(self, rhs: Matrix2x2<$T>) -> Matrix2x2<$T> {
        Matrix2x2::new(self * rhs.a1, self * rhs.a2,
                     self * rhs.b1, self * rhs.b2)
    }
}

    )*};
}

impl_mul_mat2!(u32, i32, f32, f64);

impl<T: Num> VectorSpace<T> for Matrix2x2<T>
where T: Mul<Self, Output=Self> {}

// impl<T> Monoid<<Mul<Self, Output = Self>>::mul> for Matrix2x2<T> {
// impl<T> Monoid<<Matrix2x2<T> as Mul<Self, Output=Self>>::mul> for Matrix2x2<T> {

// }

pub type mat2f = Matrix2x2<f32>;
pub type mat2d = Matrix2x2<f64>;
pub type mat2u = Matrix2x2<u32>;
pub type mat2i = Matrix2x2<i32>;

trait VectorSpace<T>: Add + Sized
where T: Mul<Self, Output=Self> {}

trait Monoid <M>: Sized 
where M: Fn(Self, Self) -> Self {
    fn id() -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ops() {
        let m = mat2i::new(1, 1,
                           0, 1);
        println!("{:?}", -m * m);
    }
}
