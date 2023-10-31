use std::fmt::{Display, Debug};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

/// Represents a two-dimensional vector.
#[derive(Copy, Clone, PartialEq)]
pub struct Vector2d(pub f64, pub f64);

impl Debug for Vector2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Vector2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

impl std::marker::StructuralEq for Vector2d {}

impl Vector2d {
    pub const fn zero() -> Self {
        Self(0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn set_x(&mut self, value: f64) {
        self.0 = value;
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn set_y(&mut self, value: f64) {
        self.1 = value;
    }

    pub fn dot_product(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }

    /*
    pub(crate) fn to_nalgebra_point(&self) -> nalgebra::geometry::Point2<f64> {
        nalgebra::geometry::Point2::new(self.x(), self.y())
    }

    pub(crate) fn from_nalgebra_point(point: nalgebra::geometry::Point2<f64>) -> Self {
        Self(point.x, point.y)
    }
    */
}

impl Add<Vector2d> for Vector2d {
    type Output = Vector2d;
    fn add(self, rhs: Vector2d) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vector2d {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Vector2d> for Vector2d {
    type Output = Vector2d;
    fn sub(self, rhs: Vector2d) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vector2d {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<Vector2d> for Vector2d {
    type Output = Vector2d;
    fn mul(self, rhs: Vector2d) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl MulAssign for Vector2d {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}

impl Div<Vector2d> for Vector2d {
    type Output = Vector2d;
    fn div(self, rhs: Vector2d) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl DivAssign for Vector2d {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
    }
}

impl Add<f64> for Vector2d {
    type Output = Vector2d;
    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs)
    }
}

impl AddAssign<f64> for Vector2d {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
        self.1 += rhs;
    }
}

impl Sub<f64> for Vector2d {
    type Output = Vector2d;
    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs)
    }
}

impl SubAssign<f64> for Vector2d {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
        self.1 -= rhs;
    }
}

impl Mul<f64> for Vector2d {
    type Output = Vector2d;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl MulAssign<f64> for Vector2d {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl Div<f64> for Vector2d {
    type Output = Vector2d;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl DivAssign<f64> for Vector2d {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}