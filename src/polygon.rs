
use super::Point;

#[derive(Clone)]
pub struct Polygon {
    points: Vec<Point>,
    filled: bool,  // Set if polygon is filled
}

