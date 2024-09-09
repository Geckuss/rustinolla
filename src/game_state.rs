use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::{WIDTH, HEIGHT, BLUE, RED};
use crate::Render;
use crate::engine::draw_circle;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Player {
    X,
    O
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GameState {
    pub grid: [[Option<Player>; 3]; 3],
    pub turn: Player
}

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
        // TODO (teht. 2): Toteuta funktio.


        None
    }
}

impl Render for GameState {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for x in 0..3_u32 {
            for y in 0..3_u32 {
                if let Some(player) = self.grid[x as usize][y as usize] {
                    
                    // Lasketaan ruudun keskikohta (pikselikoordinaatit)
                    let x = WIDTH/3*x + WIDTH/6;
                    let y = HEIGHT/3*y + HEIGHT/6;

                    // Valitaan väri pelaajan mukaan
                    canvas.set_draw_color(match player {
                        Player::O => BLUE,
                        Player::X => RED,
                    });

                    draw_circle(x as i32, y as i32, 20, canvas)?;
                }
            }
        }

        Ok(())
    }
}