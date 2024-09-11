// Projektissa käytettyjä moduuleja
mod engine;
mod game_state;

// Importteja
use engine::Engine;
use game_state::GameState;
use rustinolla::EngineEvent;


pub fn main() -> Result<(), String>{
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