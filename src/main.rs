mod engine;
use crate::engine::{Engine, EngineEvent, Render};
use sdl2::pixels::Color;

mod game_state;
use game_state::{GameState, Player};

const RED: Color = Color::RGB(255, 0, 0);
const BLUE: Color = Color::RGB(0, 0, 255);
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;


pub fn main() -> Result<(), String>{
    println!("{:?}", Player::X);
    let mut game_state = GameState::empty();
    let mut engine = Engine::init()?;
    let mut running = true;

    // main loop
    while running {
        let engine_event = engine.poll();
        match engine_event {
            EngineEvent::Click(x, y) => {
                game_state.set_square(x, y, game_state.turn);
                if let Some(winner) = game_state.has_a_winner() {
                    println!("{:?} voitti!", winner);
                    running = false
                } else if game_state.is_full() {
                    println!("Tie!");
                    game_state.clear_board()
                }
            },
            EngineEvent::Exit => running = false,
            // TODO (teht. 1): Jos engine.poll() palauttaa EngineEvent::Clear, tyhjennä lauta.
            _ => {},
        }
        engine.render(&game_state)?;
    }

  Ok(())
}