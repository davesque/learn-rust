extern crate num;

use self::num::Num;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Scalar<T> {
    pub x: T,
}

pub enum S<T> {
    S(T),
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

impl<T: Num + Copy> Mul<Vector3D<T>> for S<T> {
    type Output = Vector3D<T>;

    fn mul(self, other: Vector3D<T>) -> Self::Output {
        match self {
            S::S(x) => Self::Output::new(other.x * x, other.y * x, other.z * x),
        }
    }
}

impl<T: Num + Copy> Div<T> for Vector3D<T> {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Self::Output::new(self.x / other, self.y / other, self.z / other)
    }
}
