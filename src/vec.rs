use std::{
    cmp::PartialOrd,
    ops::{Add, Mul, Sub},
};

pub type Vec3f = Vec3<f32>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    pub fn tuple(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T: Add<Output = T> + Mul<Output = T>> Vec3<T> {
    pub fn dot(self, b: Vec3<T>) -> T {
        dot(self, b)
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Vec3<T> {
    pub fn dot2(self) -> T {
        dot(self, self)
    }
}

impl Vec3f {
    pub fn norm(self) -> f32 {
        norm(self)
    }

    pub fn interpolate(self, v: Vec3f, k: f32) -> Vec3f {
        interpolate(self, v, k)
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, _rhs: T) -> Vec3<T> {
        Vec3::new(self.x + _rhs, self.y + _rhs, self.z + _rhs)
    }
}

impl<T: Add<Output = T>> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, _rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, _rhs: T) -> Vec3<T> {
        Vec3::new(self.x - _rhs, self.y - _rhs, self.z - _rhs)
    }
}

impl<T: Sub<Output = T>> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, _rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, _rhs: T) -> Vec3<T> {
        Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
    }
}

impl<T: Mul<Output = T>> Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, _rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x * _rhs.x, self.y * _rhs.y, self.z * _rhs.z)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<&Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, _rhs: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x * _rhs.x, self.y * _rhs.y, self.z * _rhs.z)
    }
}

pub fn dot<T: Add<Output = T> + Mul<Output = T>>(a: Vec3<T>, b: Vec3<T>) -> T {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn maximum<T: PartialOrd>(a: Vec3<T>, b: Vec3<T>) -> Vec3<T> {
    let (xa, ya, za) = a.tuple();
    let (xb, yb, zb) = b.tuple();
    let x = if xa > xb { xa } else { xb };
    let y = if ya > yb { ya } else { yb };
    let z = if za > zb { za } else { zb };
    Vec3::new(x, y, z)
}

pub fn minimum<T: PartialOrd>(a: Vec3<T>, b: Vec3<T>) -> Vec3<T> {
    let (xa, ya, za) = a.tuple();
    let (xb, yb, zb) = b.tuple();
    let x = if xa < xb { xa } else { xb };
    let y = if ya < yb { ya } else { yb };
    let z = if za < zb { za } else { zb };
    Vec3::new(x, y, z)
}

pub fn norm(v: Vec3f) -> f32 {
    f32::sqrt(v.x * v.x + v.y * v.y + v.z * v.z)
}

pub fn interpolate(v1: Vec3f, v2: Vec3f, k: f32) -> Vec3f {
    let a = 1.0 - k;
    Vec3::new(
        v1.x * k + v2.x * a,
        v1.y * k + v2.y * a,
        v1.z * k + v2.z * a,
    )
}
