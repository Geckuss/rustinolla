use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use rustinolla::{BLUE, RED, YELLOW, WIDTH, HEIGHT};
use rustinolla::{Render, Player};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GameState {
    grid: [[Option<Player>; 3]; 3],
    turn: Player
}

impl GameState {
    pub fn has_a_winner(&self) -> Option<Player> {
        // TODO T2: Toteuta metodi.

        None
    }
}

// Render traitin implementaatio. Tässä määritellään, miten GameState piirretään näytölle.
impl Render for GameState {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        // Piiretään pelaajien merkinnät
        for x in 0..3_u32 {
            for y in 0..3_u32 {
                if let Some(player) = self.grid[x as usize][y as usize] {
                    let x = WIDTH/3*x + WIDTH/6;
                    let y = HEIGHT/3*y + HEIGHT/6;

                    canvas.set_draw_color(match player {
                        Player::O => BLUE,
                        Player::X => RED,
                    });
                    
                    match player {
                        Player::O => draw_circle(x as i32, y as i32, WIDTH/10, canvas)?,
                        Player::X => draw_x(x as i32, y as i32, WIDTH/10, canvas)?,
                    }
                }
            }
        }

        // TODO T5: Jos Pelin voittaja on selvillä, piirrä voittavan suoran päälle keltainen viiva.
        Ok(())
    }
}

// Apufunktiot

fn draw_circle(center_x: i32, center_y: i32, radius: u32, canvas: &mut Canvas<Window>)
-> Result<(), String> {
    let radius = radius as i32;
    // TODO T4: Määrittele toinen, pienempi säde "inner radius", joka määrittelee ympyrän reunan paksuuden.
    for x in center_x-radius..center_x+radius {
        for y in center_y-radius..center_y+radius {
            // TODO T4: Lisää if-lauseeseen inner radius siten, että sitä lähempänä olevat pikselit jätetään rauhaan.
            if distance((x, y), (center_x, center_y)) <= radius {
                canvas.draw_point((x, y))?;
            }
        }
    }
    Ok(())
}

fn draw_x(center_x: i32, center_y: i32, size: u32, canvas: &mut Canvas<Window>) 
-> Result<(), String> {
    let size = size as i32;
    /* TODO T3: 
        Toteuta funktio, joka piirtää haluamasi kuvion. Tämän pohjan for-loopit käyvät läpi
        jokaisen pikselin koordinaatit parametreissä annettujen arvojen mukaan. Niiden sisälle
        voit keksiä erilaisia sääntöjä, joiden perusteella annettu pikseli väritetään.
    */
    for x in center_x-size..center_x+size {
        for y in center_y-size..center_y+size {
            // Poista tämä if-blokki ennen kuin aloitat
            if center_x == x && center_y == y {
                draw_circle(x, y, size as u32, canvas)?;
            }
        }
    }
    Ok(())
}


// -------------------------------------------------
// Tästä alaspäin ei tarvitse tehdä mitään muutoksia

impl GameState {
    pub fn set_square(&mut self, x: i32, y: i32) {
        if x < 3 && y < 3 && self.grid[x as usize][y as usize].is_none() {
            self.grid[x as usize][y as usize] = Some(self.turn);
            self.switch_turns()
        }
    }

    pub fn empty() -> Self {
        GameState { 
            grid: [[None; 3]; 3],
            turn: Player::X
        }
    }

    pub fn reset(&mut self) {
        self.grid = [[None; 3]; 3];
        self.turn = Player::X;
    }

    fn switch_turns(&mut self) {
        self.turn = match self.turn {
            Player::O => Player::X,
            Player::X => Player::O
        }
    }

    pub fn is_full(&self) -> bool {
        self.grid
            .iter()
            .filter(|row| row.contains(&None))
            .count() == 0
    }    
}
pub fn draw_bar(start: (i32, i32), end: (i32, i32), canvas: &mut Canvas<Window>,)
-> Result<(), String> {
    let padding = 30_i32;
    let orientation = get_bar_orientation(start, end);
    let sx = match start.0 < end.0 {
        true => start.0,
        false => end.0,
    };
    let sy = match start.1 < end.1 {
        true => start.1,
        false => end.1
    };
    let ex = match start.0 > end.0 {
        true => start.0,
        false => end.0,
    };
    let ey = match start.1 > end.1 {
        true => start.1,
        false => end.1
    };
    match orientation {
        BarOrientation::Horizontal => {
            let x = WIDTH/3*sx as u32 + padding as u32;
            let y = HEIGHT/3*sy as u32 + HEIGHT/6;
            let rect = Rect::new(
                x as i32, 
                y as i32,
                WIDTH - padding as u32*2,
                HEIGHT/100,
            );
            canvas.fill_rect(rect)?;
        },
        BarOrientation::Vertical => {
            let x = WIDTH/3*sx as u32 + HEIGHT/6;
            let y = HEIGHT/3*sy as u32 + padding as u32;
            let rect = Rect::new(
                x as i32, 
                y as i32,
                WIDTH/100,
                HEIGHT - padding as u32*2,
            );
            canvas.fill_rect(rect)?;
        },
        BarOrientation::Diagonal => {
            for i in -(WIDTH as i32/130)..WIDTH as i32/130 {
                let start = (
                    WIDTH as i32/3*sx + i + padding,
                    HEIGHT as i32/3*sy + padding,
                );
        
                let end = (
                    WIDTH as i32/3*(ex+1) + i - padding,
                    HEIGHT as i32/3*(ey+1) - padding,
                );

                canvas.draw_line(start, end)?;
            }
        }
    };

    Ok(())
}

fn distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    f32::sqrt((p2.0-p1.0).pow(2) as f32 + (p2.1-p1.1).pow(2) as f32) as i32
}

#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum BarOrientation {
    Horizontal,
    Vertical,
    Diagonal,
}

#[allow(unused)]
fn get_bar_orientation(start: (i32, i32), end: (i32, i32))
 -> BarOrientation {
    use BarOrientation::*;
    match (start, end) {
        ((0, 0), (2, 0)) | ((2, 0), (0, 0)) => Horizontal,
        ((0, 1), (2, 1)) | ((2, 1), (0, 1)) => Horizontal,
        ((0, 2), (2, 2)) | ((2, 2), (0, 2)) => Horizontal,

        ((0, 0), (0, 2)) | ((0, 2), (0, 0)) => Vertical,
        ((1, 0), (1, 2)) | ((1, 2), (1, 0)) => Vertical,
        ((2, 0), (2, 2)) | ((2, 2), (2, 0)) => Vertical,

        ((0, 0), (2, 2)) | ((2, 2), (0, 0)) => Diagonal,
        ((0, 2), (2, 0)) | ((2, 0), (0, 2)) => Diagonal,
        _ => panic!(
"\nEt voi piirtää viivaa pisteestä {:?} pisteeseen {:?}.
Viivan täytyy olla pystysuora, vaakasuora tai kulkea kulmasta kulmaan.\n", 
        start, end
        ),
    }
}