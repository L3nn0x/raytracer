use std::ops::{Neg, AddAssign, SubAssign, MulAssign, DivAssign, Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn make_unit(&mut self) {
        *self /= self.length();
    }
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3{
        x: v1.y * v2.z - v1.z * v2.y,
        y: -(v1.x * v2.z - v1.z * v2.x),
        z: v1.x * v2.y - v1.y * v2.x
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    let mut tmp = v.clone();
    tmp.make_unit();
    tmp
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f32) -> Vec3 {
        Vec3{
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        self.x /= t;
        self.y /= t;
        self.z /= t;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }
}

#[test]
fn vec3_neg() {
    let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    let a = -a;
    let c = Vec3{x: -1.0, y: -2.0, z: -3.0};
    assert!(a == c);
}

#[test]
fn vec3_add_assign() {
    let mut a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    a += Vec3{x: 2.0, y: 3.0, z: 4.0};
    let b = Vec3{x: 3.0, y: 5.0, z: 7.0};
    assert!(a == b);
}

#[test]
fn vec3_sub_assign() {
    let mut a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    a -= Vec3{x: 2.0, y: 3.0, z: 4.0};
    let b = Vec3{x: -1.0, y: -1.0, z: -1.0};
    assert!(a == b);
}

#[test]
fn vec3_mul_assign() {
    let mut a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    a *= Vec3{x: 2.0, y: 3.0, z: 4.0};
    let b = Vec3{x: 2.0, y: 6.0, z: 12.0};
    assert!(a == b);
    a *= 2.0;
    assert!(a == Vec3{x: 4.0, y: 12.0, z: 24.0});
}

#[test]
fn vec3_div_assign() {
    let mut a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    a /= Vec3{x: 2.0, y: 3.0, z: 4.0};
    let b = Vec3{x: 0.5, y: 2.0/3.0, z: 3.0/4.0};
    assert!(a == b);
    a /= 2.0;
    assert!(a == Vec3{x: 0.25, y: 1.0/3.0, z: 3.0/8.0});
}

#[test]
fn vec3_add() {
    let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    assert!(a + a == Vec3{x: 2.0, y: 4.0, z: 6.0});
}

#[test]
fn vec3_sub() {
    let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    assert!(a - a == Vec3{x: 0.0, y: 0.0, z: 0.0});
}

#[test]
fn vec3_mul() {
    let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    assert!(a * a == Vec3{x: 1.0, y: 4.0, z: 9.0});
    assert!(a * 2.0 == Vec3{x: 2.0, y: 4.0, z: 6.0});
}

#[test]
fn vec3_div() {
    let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    assert!(a / a == Vec3{x: 1.0, y: 1.0, z: 1.0});
}

#[test]
fn vec3_dot() {
    let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    let b = Vec3{x: 2.0, y: 3.0, z: 4.0};
    assert!(dot(&a, &b) == 20.0);
}

#[test]
fn vec3_cross() {
    let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    let b = Vec3{x: 2.0, y: 3.0, z: 4.0};
    assert!(cross(&a, &b) == Vec3{x: -1.0, y: 2.0, z: -1.0});
}

#[test]
fn vec3_length() {
    let a = Vec3{x: 1.0, y: 1.0, z: 1.0};
    assert!(a.length() == (3.0 as f32).sqrt());
}

#[test]
fn vec3_squared_length() {
    let a = Vec3{x: 1.0, y: 1.0, z: 1.0};
    assert!(a.squared_length() == 3.0);
}

#[test]
fn vec3_make_unit() {
    let mut a = Vec3{x: 1.0, y: 0.0, z: 0.0};
    a.make_unit();
    assert!(a.length() == 1.0);
    let mut a = Vec3{x: 0.0, y: 1.0, z: 0.0};
    a.make_unit();
    assert!(a.length() == 1.0);
    let mut a = Vec3{x: 0.0, y: 0.0, z: 1.0};
    a.make_unit();
    assert!(a.length() == 1.0);
}

#[test]
fn vec3_unit_vector() {
    let a = Vec3{x: 1.0, y: 0.0, z: 0.0};
    let a = unit_vector(&a);
    assert!(a.length() == 1.0);
}
