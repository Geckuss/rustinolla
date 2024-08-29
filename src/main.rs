
use sdl2::{render::Canvas, pixels::Color};
use sdl2::video::Window;

mod engine;
use crate::engine::{Engine, EngineEvent, Render};

mod game_state;
use game_state::GameState;

const RED: Color = Color::RGB(255, 0, 0);
const BLUE: Color = Color::RGB(0, 0, 255);
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

pub fn main() -> Result<(), String>{
    println!("{}", distance((10, 15), (40, 60)));
    let mut game_state = GameState::empty();
    let mut engine = Engine::init()?;
    let mut running = true;

    // main loop
    while running {
        match engine.poll() {
            EngineEvent::Click(x, y) => {

                game_state.set_square(x, y, game_state.turn);

                if let Some(winner) = game_state.has_a_winner() {
                    println!("{:?} won the game!", winner);
                    running = false
                } else if game_state.is_full() {
                    println!("Tie!");
                    running = false
                }
            },
            EngineEvent::Exit => running = false,
            EngineEvent::None => {},
            // TODO: Jos engine.poll() palauttaa EngineEvent::Clear, tyhjennä lauta.
            // GameState-tyyppi tarjoaa metodin clear_board().
            EngineEvent::Clear => game_state.clear_board()
        }
        engine.render(&game_state)?;
    }

  Ok(())
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

fn draw_circle(center_x: i32, center_y: i32, radius: i32, canvas: &mut Canvas<Window>)
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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum Player {
    X,
    O
}

impl GameState {
    fn has_a_winner(&self) -> Option<Player> {
        /*
        -------------- TODO --------------
        Kirjoita funktio joka tarkistaa, onko jompikumpi pelaajista voittanut pelin.

        Huom!
        Funktion palauttama Player on kääritty Option-tyyppiin. Tämä vastaa muista 
        kielistä tuttua NULL-arvoa, mitä Rustista ei löydy. Voittajan ollessa selvillä 
        palautusarvon tulisi olla Some(Player::O) tai Some(Player::X). Muuten funktio palauttaa None.
        */


        None
    }
}




