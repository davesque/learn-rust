extern crate num;

use self::num::Num;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x: x, y: y, z: z }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    pub x: T,
}

impl<T: Num> Scalar<T> {
    pub fn new(x: T) -> Self {
        Self { x: x }
    }
}

impl<T: Num> Add for Vector3D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: Num> Sub for Vector3D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T: Num + Copy> Mul<Scalar<T>> for Vector3D<T> {
    type Output = Self;

    fn mul(self, other: Scalar<T>) -> Self::Output {
        Self::Output::new(self.x * other.x, self.y * other.x, self.z * other.x)
    }
}

impl<T: Num + Copy> Mul<Vector3D<T>> for Scalar<T> {
    type Output = Vector3D<T>;

    fn mul(self, other: Vector3D<T>) -> Self::Output {
        Self::Output::new(other.x * self.x, other.y * self.x, other.z * self.x)
    }
}

impl<T: Num + Copy> Mul<T> for Vector3D<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self::Output::new(self.x * other, self.y * other, self.z * other)
    }
}

// impl<T: Num + Copy> Mul<Vector3D<T>> for T {
//     type Output = Vector3D<T>;
//
//     fn mul(self, other: Vector3D<T>) -> Self::Output {
//         Self::Output::new(other.x * self, other.y * self, other.z * self)
//     }
// }

impl Mul<Vector3D<f64>> for f64 {
    type Output = Vector3D<f64>;

    fn mul(self, other: Vector3D<f64>) -> Self::Output {
        Self::Output::new(other.x * self, other.y * self, other.z * self)
    }
}
