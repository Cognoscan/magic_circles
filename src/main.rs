
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::gfx::framerate::FPSManager;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::f32::consts::PI;

use magic_circles::*;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window_width = 640;
    let window_height = 480;
    let window = video_subsystem.window("SDL2", window_width, window_height)
        .position_centered().build().map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .accelerated().build().map_err(|e| e.to_string())?;

    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(60).map_err(|e| e.to_string())?;

    let n = 12;
    let points: Vec<Point> = (0..n)
        .map(|x| Point::new(
                ((x as f32)/(n as f32)*2.0*PI).cos(),
                ((x as f32)/(n as f32)*2.0*PI).sin()))
        .collect();

    let mut lines: Vec<Line> = Vec::new();
    for i in 1..((n+1)/2) {
        lines.extend(points.iter()
            .zip(points.iter().cycle().skip(i))
            .map(|(p0, p1)| Line::from_points(*p0, *p1))
            );
    }

    let window_size = window_width.min(window_height);
    let screen_transform = Transform {
        a11: (window_size/2) as f32,
        a12: 0.0,
        a21: 0.0,
        a22: -((window_size/2) as f32),
        b: Point { x: (window_width/2) as f32, y: (window_height/2) as f32 },
    };

    let mut animation = Animation::new(AnimationStyle::FromStart, 0.0);

    let mut anim_val = 0.0;
    let mut angle = 0.0;

    let mut event_pump = sdl_context.event_pump()?;
    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        canvas.clear();

        anim_val += 1.0/60.0;
        if anim_val >= 1.0 { anim_val -= 1.0; }

        if anim_val > 0.5 {
            animation.set_state(2.0*(1.0-anim_val));
        }
        else {
            animation.set_state(2.0*anim_val);
        }

        angle = (angle + 1.0/240.0) % 1.0;

        let rot = Transform::from_angle(angle).conv(&screen_transform);

        for l in lines.iter() {
            l.draw(Color::rgb(255,255,255), &rot, &animation, &canvas)
                .map_err(|e| e.to_string())?;
        }

        fps_manager.delay();
        canvas.present();
    }

    Ok(())
}
