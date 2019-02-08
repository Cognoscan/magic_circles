use std::ops::{Add, Sub, Mul};

use super::Transform;

#[derive(Debug,Clone,Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;
    fn mul(self, rhs: f32) -> Point {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn transform(self, t: &Transform) -> Point {
        Point {
            x: t.a11 * self.x + t.a12 * self.y + t.b.x,
            y: t.a21 * self.x + t.a22 * self.y + t.b.y,
        }
    }

}

