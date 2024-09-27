use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use rustinolla::{BACKGROUND, FOREGROUND, HEIGHT, WIDTH};
use rustinolla::{Render, EngineEvent};

pub struct Engine {
  canvas: Canvas<Window>,
  event_pump: EventPump,
}

impl Engine {
    pub fn poll(&mut self) -> EngineEvent {
        match self.event_pump.wait_event() {
            Event::Quit { .. } => EngineEvent::Exit,
            Event::KeyDown { keycode, ..} => {
                if let Some(keycode) = keycode {
                    match keycode {
                        Keycode::ESCAPE => EngineEvent::Exit,
                        // TODO (teht. 1): Jos keycode on välilyönti, palauta EngineEvent::Clear
                        Keycode::SPACE => EngineEvent::Clear,
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


// -------------------------------------------------
// Tästä alaspäin ei tarvitse tehdä mitään muutoksia

impl Engine {
    pub fn init() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context
            .video()
            .unwrap();

        let window = video_subsystem
            .window("Rustinolla", WIDTH, HEIGHT)
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

    pub fn draw_backround(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(BACKGROUND);
        self.canvas.clear();
        self.canvas.set_draw_color(FOREGROUND);
        for i in 1..=2 {
            self.canvas.fill_rect(
                Rect::new((i*(WIDTH as i32/3))-5, 0, 10, HEIGHT)
            )?;
            self.canvas.fill_rect(
                Rect::new(0, (i*(HEIGHT as i32/3))-5, WIDTH, 10)
            )?;
        }

        Ok(())
    }

    pub fn render(&mut self, object: &dyn Render) -> Result<(), String> {
        object.render(&mut self.canvas)?;
        Ok(())
    }

    pub fn present(&mut self) {
        self.canvas.present()
    }
}