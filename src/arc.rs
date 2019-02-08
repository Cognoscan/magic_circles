
use super::{Point, Color, Transform, Animation, AnimationStyle, Renderer};

#[derive(Clone,Copy)]
pub struct Arc {
    center: Point, // Centerpoint of arc
    dir: Point,    // Point to start of arc
    percent: f32,  // Percent of arc (0 to 1)
}

