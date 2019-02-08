
use super::{Point, Color, Transform, Animation, AnimationStyle, Renderer};

#[derive(Debug,Clone,Copy)]
pub struct Line {
    p0: Point,
    dir: Point
}

impl Line {
    pub fn from_points(p0: Point, p1: Point) -> Line {
        let dir = p1 - p0;
        Line { p0, dir }
    }

    pub fn new(p0: Point, dir: Point) -> Line {
        Line { p0, dir }
    }

    pub fn transform(self, t: &Transform) -> Line {
        Line {
            p0: self.p0.transform(t),
            dir: Point {
                x: t.a11 * self.dir.x + t.a12 * self.dir.y,
                y: t.a21 * self.dir.x + t.a22 * self.dir.y,
            },
        }
    }

    pub fn draw(&self, color: Color, trans: &Transform, anim: &Animation, r: &Renderer) -> Result<(), String> {
        let line = self.transform(trans);
        match anim.get_style() {
            AnimationStyle::FromStart => {
                let p1 = line.p0 + (line.dir * anim.get_state());
                r.draw_line(line.p0, p1, color)
            },
            _ => {
                let p1 = line.p0 + (line.dir * anim.get_state());
                r.draw_line(line.p0, p1, color)
            }
        }
    }

}
