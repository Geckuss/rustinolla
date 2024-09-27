// Tähän tiedostoon ei tarvitse tehdä muutoksia, ellet halua muokata värejä tai ikkunan kokoa

// Importteja
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

// Globaaleja vakioita
pub const RED: Color = Color::RGB(255, 0, 0);
pub const BLUE: Color = Color::RGB(0, 0, 255);
pub const YELLOW: Color = Color::RGB(255, 255, 0);
pub const BACKGROUND: Color = Color::RGB(40, 40, 40);
pub const FOREGROUND: Color = Color::RGB(80, 80, 80);
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 800;

// Rustin versio rajapinnoista.
pub trait Render {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}

// Enum-tyyppi kuvastamaan rustinollan pelaajia
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Player {
    X,
    O,
}

// Enum-tyyppi mainloopin ja moottorin välistä kommunikaatiota varten
#[derive(Debug, Clone, Copy)]
pub enum EngineEvent {
    Click(i32, i32), //(x, y)
    Exit,
    None,
    Clear,
}
