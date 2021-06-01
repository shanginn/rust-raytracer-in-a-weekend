use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn squared_length(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn make_unit_vector(&self) -> Self {
        let k = 1.0 / self.length();

        Vec3(self.0 * k, self.1 * k, self.2 * k)
    }

    pub fn unit_vector(vector: Vec3) -> Self {
        vector / vector.length()
    }

    pub fn unit(size: f64) -> Self {
        Vec3(size, size, size)
    }

    pub fn dot(first: &Vec3, second: &Vec3) -> f64 {
        first.0 * second.0 + first.1 * second.1 + first.2 * second.2
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            -(self.0 * other.2 - self.2 * other.0),
            self.0 * other.1 - self.1 * other.0
        )
    }

    pub fn sqrt(self) -> Self {
        Vec3(
            self.0.sqrt(),
            self.1.sqrt(),
            self.2.sqrt(),
        )
    }

    pub fn x(&self) -> f64 { self.0 }
    pub fn y(&self) -> f64 { self.1 }
    pub fn z(&self) -> f64 { self.2 }

    pub fn r(&self) -> f64 { self.0 }
    pub fn g(&self) -> f64 { self.1 }
    pub fn b(&self) -> f64 { self.2 }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.0 as u8, self.1 as u8, self.2 as u8)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0, self.1 - other.1, self.2 - other.2);
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self(self.0 / other.0, self.1 / other.1, self.2 / other.2);
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self(self.0 * other.0, self.1 * other.1, self.2 * other.2);
    }
}

impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self(self.0 / other, self.1 / other, self.2 / other);
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self(self.0 * other, self.1 * other, self.2 * other);
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}