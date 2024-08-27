use rand::prelude::*;
use crate::Player;


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GameState {
    pub grid: [[Option<Player>; 3]; 3],
    pub turn: Player
}

impl GameState {

    pub fn _random() -> Self {
        let mut grid = [[None; 3]; 3];
        let mut rng = rand::thread_rng();
        for i in 0..3 {
            for j in 0..3 {
                grid[i][j] = match rng.gen_range(0..3) {
                    0 => Some(Player::X),
                    1 => Some(Player::O),
                    _ => None
                }
            }      
        }
        GameState { grid, turn: Player::X }
    }

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
