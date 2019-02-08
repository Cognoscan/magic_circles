
use super::Point;

#[derive(Clone,Copy)]
pub struct Transform {
    pub a11: f32,
    pub a12: f32,
    pub a21: f32,
    pub a22: f32,
    pub b: Point,
}

impl Transform {
    pub fn from_angle(angle: f32) -> Transform {
        let scale = 2.0 * ::std::f32::consts::PI;
        let cos = (angle * scale).cos();
        let sin = (angle * scale).sin();
        Transform {
            a11:  cos,
            a12: -sin,
            a21:  sin,
            a22:  cos,
            b: Point { x: 0.0, y: 0.0 },
        }
    }

    pub fn conv(self, other: &Transform) -> Transform {
        Transform {
            a11: self.a11 * other.a11 + self.a21 * other.a12,
            a12: self.a12 * other.a11 + self.a22 * other.a12,
            a21: self.a11 * other.a21 + self.a21 * other.a22,
            a22: self.a12 * other.a21 + self.a22 * other.a22,
            b: self.b.transform(&other),
        }
    }
}
