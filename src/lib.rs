extern crate sdl2;

//mod renderer;

//pub use renderer::renderer;

mod point;
mod line;
mod arc;
mod circle;
mod polygon;
mod transform;
mod renderer;

pub use self::point::Point;
pub use self::line::Line;
pub use self::arc::Arc;
pub use self::circle::Circle;
pub use self::polygon::Polygon;
pub use self::transform::Transform;
pub use self::renderer::Renderer;

use sdl2::gfx::primitives::ToColor;

#[derive(Clone,Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[derive(Clone, Copy)]
pub enum AnimationStyle {
    FromStart,
    FromEnd,
    FromBoth,
    FromMid,
}

#[derive(Clone,Copy)]
pub struct Animation {
    style: AnimationStyle,
    state: f32,
    var: f32,
}

impl ToColor for Color {
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }
}

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {red, green, blue, alpha: 255}
    }
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {red, green, blue, alpha}
    }
}

impl Animation {
    pub fn new(style: AnimationStyle, var: f32) -> Animation {
        Animation { style, state: 0.0, var: var.min(1.0).max(0.0) }
    }
    pub fn get_style(&self) -> AnimationStyle {
        self.style
    }
    pub fn get_var(&self) -> f32 {
        self.var
    }
    pub fn get_state(&self) -> f32 {
        self.state
    }
    pub fn set_animation(&mut self, anim: AnimationStyle) {
        self.style = anim;
    }
    pub fn set_var(&mut self, var: f32) {
        self.var = var.min(1.0).max(0.0);
    }
    pub fn set_state(&mut self, state: f32) {
        self.state = state.min(1.0).max(0.0);
    }
}
