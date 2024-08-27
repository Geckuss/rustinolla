use sdl2::rect::Rect;
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
    let mut game_state = GameState::empty();
    let mut engine = Engine::init()?;
    let mut running = true;

    // main loop
    while running {
        match engine.poll() {
            EngineEvent::Click(x, y) => {
                /* Jos paikkalla on tilaa (self.grid[x][y] == None),
                set_square() vaihtaa vuoron toiselle pelaajalle */
                println!("Clicked [{x}][{y}]");
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
            EngineEvent::None => {}
        }
        engine.render(&game_state)?;
    }

  Ok(())
}


impl Render for GameState {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {

        /*
        -------------- TODO ---------------
        Korvaa funktion toteutus uudella koodilla, joka renderöi self.gridin 
        tilan mukaisesti X ja O oikeisiin kohtiin.

        For loopit käyvät läpi kaikkien ruutujen indeksit [0][1], [0][2], .., [2][2].
        Voit käyttää tätä pohjana omalle toteutuksellesi.
         */
        for x in 0..3_u32 {
            for y in 0..3_u32 {
                // "Jos self.grid[x][y] on merkitty jollekin pelaajalle {..}"
                if let Some(player) = self.grid[x as usize][y as usize] {
                    
                    // Lasketaan ruudun keskikohta (pikselikoordinaatit)
                    let x = WIDTH/3*x + HEIGHT/6;
                    let y = HEIGHT/3*y + HEIGHT/6;
                    
                    // Luodaan 50x50 pikselin kokoinen neliö.
                    let mut rect = Rect::new(0, 0, 50, 50);

                    // Siirretään neliö aiemmin laskettuihin koordinaatteihin
                    Rect::center_on(&mut rect, (x as i32, y as i32));


                    // Valitaan väri pelaajan mukaan
                    canvas.set_draw_color(match player {
                        Player::O => BLUE,
                        Player::X => RED,
                    });

                    // Taytetään neliö valitulla värillä
                    canvas.fill_rect(rect)?;
                }
            }
        }
        Ok(())
    }
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




