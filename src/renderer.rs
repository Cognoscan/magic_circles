
use super::{Point, Color};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::{Canvas, RenderTarget};

pub trait Renderer {
    fn draw_point(&self, pt: Point, color: Color) -> Result<(), String>;
    fn draw_line(&self, p0: Point, p1: Point, color: Color) -> Result<(), String>;
    fn draw_arc(&self, center: Point, radius: f32, start: f32, end: f32, color: Color) -> Result<(), String>;
    fn draw_circle(&self, center: Point, radius: f32, color: Color) -> Result<(), String>;
    fn draw_filled_circle(&self, center: Point, radius: f32, bg_color: Color, color: Color) -> Result<(), String>;
    fn draw_polygon(&self, points: &Vec<Point>, color: Color) -> Result<(), String>;
    fn draw_filled_polygon(&self, points: &Vec<Point>, bg_color: Color, color: Color) -> Result<(), String>;
}

impl<T> Renderer for Canvas<T> where T: RenderTarget {
    fn draw_point(&self, pt: Point, color: Color) -> Result<(), String> {
        self.pixel(pt.x as i16, pt.y as i16, color)
    }
    fn draw_line(&self, p0: Point, p1: Point, color: Color) -> Result<(), String> {
        self.line(p0.x as i16, p0.y as i16, p1.x as i16, p1.y as i16, color)
    }
    fn draw_arc(&self, center: Point, radius: f32, start: f32, end: f32, color: Color) -> Result<(), String> {
        self.arc(center.x as i16, center.y as i16, radius as i16, (start*360.0) as i16, (end*360.0) as i16, color)
    }
    fn draw_circle(&self, center: Point, radius: f32, color: Color) -> Result<(), String> {
        self.circle(center.x as i16, center.y as i16, radius as i16, color)
    }
    fn draw_filled_circle(&self, center: Point, radius: f32, bg_color: Color, color: Color) -> Result<(), String> {
        self.filled_circle(center.x as i16, center.y as i16, radius as i16, bg_color)?;
        self.circle(center.x as i16, center.y as i16, radius as i16, color)
    }
    fn draw_polygon(&self, points: &Vec<Point>, color: Color) -> Result<(), String> {
        let vx: Vec<i16> = points.iter().map(|p| p.x as i16).collect();
        let vy: Vec<i16> = points.iter().map(|p| p.y as i16).collect();
        self.polygon(&vx[..], &vy[..], color)
    }
    fn draw_filled_polygon(&self, points: &Vec<Point>, bg_color: Color, color: Color) -> Result<(), String> {
        let vx: Vec<i16> = points.iter().map(|p| p.x as i16).collect();
        let vy: Vec<i16> = points.iter().map(|p| p.y as i16).collect();
        self.filled_polygon(&vx[..], &vy[..], bg_color)?;
        self.polygon(&vx[..], &vy[..], color)
    }
}

