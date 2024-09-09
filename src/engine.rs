use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::pixels::Color;
use crate::{WIDTH, HEIGHT};

pub struct Engine {
  canvas: Canvas<Window>,
  event_pump: EventPump,
}

impl Engine {
    pub fn init() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context
            .video()
            .unwrap();

        let window = video_subsystem
            .window("Pöhinä", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .unwrap();

        let event_pump = sdl_context.event_pump()?;

        Ok(Engine {
            canvas,
            event_pump
        })
    }

    pub fn render(&mut self, render_object: &dyn Render) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(40, 40, 40));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(80, 80, 80));
        for i in 1..=2 {
            self.canvas.fill_rect(
                Rect::new((i*(WIDTH as i32/3))-5, 0, 10, HEIGHT)
            )?;
            self.canvas.fill_rect(
                Rect::new(0, (i*(HEIGHT as i32/3))-5, HEIGHT, 10)
            )?;
        }
        render_object.render(&mut self.canvas)?;
        self.canvas.present();
        Ok(())
    }

    pub fn poll(&mut self) -> EngineEvent {
        match self.event_pump.wait_event() {
            Event::Quit { .. } => EngineEvent::Exit,
           
            Event::KeyDown { keycode, ..} => {
                if let Some(key) = keycode {
                    match key {
                        Keycode::ESCAPE => EngineEvent::Exit,
                    // TODO (teht. 1): Jos key on välilyönti, palauta EngineEvent::Clear
                        _ => EngineEvent::None
                    }
                } else {
                    EngineEvent::None
                }
            }

            Event::MouseButtonDown { mouse_btn, mut x,  mut y, .. } => {
                if mouse_btn == MouseButton::Left {
                    if x < WIDTH as i32/3 { x = 0 }
                    else if x < WIDTH as i32/3*2 { x = 1 }
                    else { x = 2 }

                    if y < HEIGHT as i32/3 { y = 0 }
                    else if y < HEIGHT as i32/3*2 { y = 1 }
                    else { y = 2 }

                    return EngineEvent::Click(x, y)
                } 
                EngineEvent::None
            }
        _ => EngineEvent::None 
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EngineEvent {
    Click(i32, i32), //(x, y)
    Exit,
    None,
    Clear,
}

pub trait Render {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}

pub fn draw_circle(center_x: i32, center_y: i32, radius: i32, canvas: &mut Canvas<Window>)
-> Result<(), String> {
    for x in center_x-radius..center_x+radius {
        for y in center_y-radius..center_y+radius {
            if distance((x, y), (center_x, center_y)) <= radius {
                canvas.draw_point((x, y))?;
            }
        }
    }

    Ok(())
}

fn distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    f32::sqrt((p2.0-p1.0).pow(2) as f32 + (p2.1-p1.1).pow(2) as f32) as i32
}