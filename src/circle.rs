
use super::Point;

#[derive(Clone,Copy)]
pub struct Circle {
    center: Point, // Centerpoint of arc
    radius: f32,   // Radius of circle
    filled: bool,  // Set if circle is filled
}

