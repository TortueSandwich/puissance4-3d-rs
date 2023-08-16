use std::cell::Cell;

use crossterm::{
    event::{Event, KeyCode},
    terminal::disable_raw_mode,
};

use crate::plate::PreviewAction;
use crate::{
    coord::Letter, error::P4Error, input::Input, plate::Plate, player::PlayerID,
    utils::CloseTerminal, Game,
};

use rand::Rng;

pub trait Thinker {
    fn think(&self, plt: &Plate) -> Result<Input, P4Error>;
}

pub struct RBot;
impl Thinker for RBot {
    fn think(&self, plt: &Plate) -> Result<Input, P4Error> {
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen_range(0..=15);
        let res: Input;
        loop {
            let input = Input::from(random_number);
            if plt.playable(input.clone()) {
                res = input;
                break;
            }
        }
        Ok(res)
    }
}

// pub struct RBotSmarter;
// impl Thinker for RBotSmarter {
//     fn think(&self, plt: &Plate) -> Result<Input, P4Error> {
//         let mut rng = rand::thread_rng();
//         let random_number: u8 = rng.gen_range(0..=15);
//         let mut res = Input::default();
//         loop {
//             let input = Input::from(random_number);
//             if plt.playable(input.clone()) {
//                 res = input;
//                 break;
//             }
//         }
//         Ok(res)
//     }
// }

#[derive(Clone, Debug)]
pub struct Human {
    pub player_id: PlayerID,
    last_message_length : Cell<usize>,
}
impl Human {
    pub fn new(player_id : PlayerID) -> Human {
        Human {player_id, last_message_length: 0.into()}
    }
}
impl Thinker for Human {
    fn think(&self, plt: &Plate) -> Result<Input, P4Error> {
        plt.plot();
        let mut res = Input::default();
        let mut prec_prev = res.clone();
        
        Game::input_display(res.clone())?;
        crossterm::terminal::enable_raw_mode().unwrap();
        loop {
            let key_event = crossterm::event::read();
            if key_event.is_err() {
                continue;
            }

            let key_event = key_event.unwrap();
            let key_ev = match key_event {
                Event::Key(k) => k,
                _ => continue,
            };
            let keycode = key_ev.code;

            match keycode {
                KeyCode::Char('A') | KeyCode::Char('a') => res.set_letter(Letter::A),
                KeyCode::Char('B') | KeyCode::Char('b') => res.set_letter(Letter::B),
                KeyCode::Char('C') | KeyCode::Char('c') => res.set_letter(Letter::C),
                KeyCode::Char('D') | KeyCode::Char('d') => res.set_letter(Letter::D),

                KeyCode::Char('0') => res.set_number(0),
                KeyCode::Char('1') => res.set_number(1),
                KeyCode::Char('2') => res.set_number(2),
                KeyCode::Char('3') => res.set_number(3),
                KeyCode::Char('s') => {
                    plt.switch_view();
                    plt.clear_plot();
                    plt.plot()
                }
                KeyCode::Enter => {
                    if res.is_valid() {
                        if plt.playable(res.clone()) {
                            break; 
                        } else {
                            let msg = "La colonne choisie est deja pleine";
                            Game::message_display(msg)?;
                            self.last_message_length.set(msg.len()); 
                            continue;
                        }

                    }
                    else { 
                        let msg = "Tu dois entrer un input corect";
                        Game::message_display(msg)?;
                        self.last_message_length.set(msg.len()); 
                        continue;
                    }
                }
                KeyCode::Esc => {
                    crossterm::execute!(std::io::stdout(), CloseTerminal)
                        .map_err(|e| P4Error::OutputInterfaceError(e))?;
                    disable_raw_mode().expect("j'ai pas reussi frerre");
                    std::process::exit(0);
                }
                _ => {}
            }
            Game::input_display(res.clone())?;
            if !(res.is_valid() && plt.playable(res.clone())) { continue; }
            plt.preview(res.clone(), PreviewAction::Add)?;
            if prec_prev.is_valid() && (res != prec_prev) {
                plt.preview(prec_prev.clone(), PreviewAction::Remove)
                    .expect("error when cleaning preview");
            }
            prec_prev = res.clone();
            Game::message_display(" ".repeat(self.last_message_length.get()).as_str())?;
            self.last_message_length.set(0); 
        }

        crossterm::terminal::disable_raw_mode().unwrap();

        Ok(res)
    }
}
