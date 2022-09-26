
use std::ops::{Add, Mul, Sub, Neg, AddAssign, SubAssign};

use num::Num;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector2< T> {
    pub x: T, pub y: T,
    // struct {u: T, v: T},
    // buffer: [T; 2],
}

impl<T: Num> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {x,y}
    }

    pub fn zero() -> Self {
        Self::new(T::zero(),T::zero())
    }
    pub fn one() -> Self {
        Self::new(T::one(), T::one())
    }
}

impl<T: Num> Add<Self> for Vector2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Num + AddAssign> AddAssign<Self> for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x; self.y += rhs.y;
    }
}

impl<T: Num> Sub<Self> for Vector2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl<T: Num + SubAssign> SubAssign<Self> for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x; self.y -= rhs.y;
    }
}
impl<T: Num + Neg<Output=T>> Neg for Vector2<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl<T> AsMut<[T; 2]> for Vector2<T> {
    fn as_mut(&mut self) -> &mut [T; 2] {
        return unsafe {&mut *(self as *mut Vector2<T> as *mut [T; 2])};
    }
}

macro_rules! impl_mul_vec2 {
    ($($T:ty),*) => {$(

impl Mul<Vector2<$T>> for $T {
    type Output = Vector2<$T>;
    fn mul(self, rhs: Vector2<$T>) -> Vector2<$T> {
        Vector2::new(self * rhs.x, self * rhs.y)
    }
}

    )*};
}

impl_mul_vec2!(f32, f64, i32, u32);

impl<T: Num> VectorSpace<T> for Vector2<T>
where T: Mul<Self, Output=Self> {}

pub type vec2f = Vector2<f32>;
pub type vec2d = Vector2<f64>;
pub type vec2u = Vector2<u32>;
pub type vec2i = Vector2<i32>;

trait VectorSpace<T>: Add + Sized
where T: Mul<Self, Output=Self> {}


#[derive(Clone, Copy, Debug)]
pub struct Vector4<T> {
    x: T, y: T, z: T, w: T,
}

impl<T: Num> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {x,y,z,w}
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }
    pub fn one() -> Self {
        Self::new(T::one(), T::one(), T::one(), T::one())
    }
}

impl<T: Num> Add<Self> for Vector4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y,
                  self.z + rhs.z, self.w + rhs.w,)
    }
}
impl<T: Num + Copy> Sub<Self> for Vector4<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y,
                  self.x - rhs.x, self.y - rhs.y,)
    }
}
impl<T: Num + Neg<Output=T>> Neg for Vector4<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y,
                  -self.z, -self.w,)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ops() {
        let v = vec2i::new(3, 4);
        println!("{:?}", -v);
    }
}
