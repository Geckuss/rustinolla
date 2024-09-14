use sdl2::render::Canvas;
use sdl2::video::Window;
use rustinolla::{WIDTH, HEIGHT, BLUE, RED};
use rustinolla::{Render, Player};

// Deklaraatio
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GameState {
    pub grid: [[Option<Player>; 3]; 3],
    pub turn: Player
}

// Metodit
impl GameState {
    pub fn set_square(&mut self, x: i32, y: i32, player: Player) {
        let x = x as usize;
        let y = y as usize;
        if x < 3 && y < 3 && self.grid[x][y].is_none() {
            self.grid[x as usize][y as usize] = Some(player);
            self.switch_turns()
        }
    }

    pub fn empty() -> Self {
        GameState { 
            grid: [[None; 3]; 3],
            turn: Player::X
        }
    }

    pub fn clear_board(&mut self) {
        self.grid = [[None; 3]; 3]
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
    
    pub fn has_a_winner(&self) -> Option<Player> {
        // TODO T2: Toteuta metodi.

        None
    }
}

// Render traitin implementaatio
impl Render for GameState {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
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
                        Player::X =>      draw_x(x as i32, y as i32, WIDTH/10, canvas)?,
                    }
                }
            }
        }

        Ok(())
    }
}

// Apufunktiot
fn draw_circle(center_x: i32, center_y: i32, radius: u32, canvas: &mut Canvas<Window>)
-> Result<(), String> {
    let radius = radius as i32;
    for x in center_x-radius..center_x+radius {
        for y in center_y-radius..center_y+radius {
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
        jokaisen pikselin koordinaatit parametreissä annettujen tietojen mukaan. Niiden sisälle
        voit keksiä erilaisia sääntöjä, joiden perusteella annettu pikseli väritetään.
    */
    for x in center_x-size..center_x+size {
        for y in center_y-size..center_y+size {
            // Poista tämä ennen kuin aloitat
            if x == center_x && y == center_y {
                draw_circle(center_x, center_y, size as u32, canvas)?;
            }
        }
    }
    Ok(())
}

fn distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    f32::sqrt((p2.0-p1.0).pow(2) as f32 + (p2.1-p1.1).pow(2) as f32) as i32
}