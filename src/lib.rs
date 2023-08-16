//#![allow(unused_imports, dead_code, unused_variables)]
use crate::{
    error::P4Error,
    logic::{Human, Thinker},
    plate::Plate,
    player::PlayerID,
    utils::{CloseTerminal, CreateTerminal, DrawSquare, DrawSquareAt, PrintAt},
};
use crossterm::cursor::{RestorePosition, SavePosition};
use crossterm::{execute, queue, style::Print};
use input::Input;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use logic::RBot;
use std::{
    io::{self, Write},
    thread,
};

pub struct Game {
    plateau: Plate,
    next_player: PlayerID,
    player_one: Box<dyn Thinker>,
    player_two: Box<dyn Thinker>,
}

impl Game {
    fn get_player(&self) -> &dyn Thinker {
        let res = match self.next_player {
            PlayerID::P1 => self.player_one.as_ref().clone(),
            PlayerID::P2 => self.player_two.as_ref().clone(),
            PlayerID::Empty => unreachable!("An empty player cant exist (get_player)"),
        };
        res
    }

    fn collect_input(&self) -> Input {
        let player = self.get_player();
        Game::next_player_display(self.next_player).unwrap();
        let intput = player.think(&self.plateau);
        match intput {
            Ok(i) => i,
            Err(e) => panic!(
                "Error when collecting an input from {} {}",
                self.next_player, e
            ),
        }
    }

    fn play_input(&mut self) {
        let lc = self.collect_input();
        self.plateau
            .add_player(lc.clone(), self.next_player)
            .expect("Error when playing input");
        Self::log_placement_display(&self, lc.clone()).unwrap();
        self.next_player.joue();
    }

    fn win(&self) -> bool {
        self.plateau.check_win()
    }

    fn run(&mut self) {
        while !self.win() {
            self.play_input();
            self.plateau.plot();
        }
        self.end();
    }

    fn end(&mut self) {
        let s1 = String::from(format!("{} à perdu", self.next_player));
        self.next_player.joue();
        let s2 = String::from(format!("{} à Gagné", self.next_player));
        let message = String::from(format!("{} {}", s1, s2));
        Self::message_display(message.as_str()).unwrap();
    }
}

impl Game {
    const CURRENT_INPUT_DISPLAY_POSITION: (u16, u16) = (15, 6);
    fn input_display(input: Input) -> Result<(), P4Error> {
        let (x, y) = Self::CURRENT_INPUT_DISPLAY_POSITION;
        queue!(
            io::stdout(),
            SavePosition,
            PrintAt(x, y, input.to_string()),
            RestorePosition
        )?;
        io::stdout().flush()?;
        Ok(())
    }
    const MESSAGE_DISPLAY_POSITION: (u16, u16) = (15, 8);
    fn message_display(message: &str) -> Result<(), P4Error> {
        let (x, y) = Self::MESSAGE_DISPLAY_POSITION;
        queue!(
            io::stdout(),
            SavePosition,
            PrintAt(x, y, message),
            RestorePosition
        )?;
        io::stdout().flush()?;
        Ok(())
    }

    const LOG_PLACEMENT_POSITION: (u16, u16) = (15, 1);
    fn log_placement_display(&self, input: Input) -> Result<(), P4Error> {
        let (x, y) = Self::LOG_PLACEMENT_POSITION;
        queue!(
            io::stdout(),
            SavePosition,
            DrawSquareAt(x, y, self.next_player.color()),
            Print(format!(" a joué en {}", input).as_str()),
            RestorePosition
        )?;
        io::stdout().flush().unwrap();
        Ok(())
    }
    const NEXT_PLAYER_DISPLAY_POSITION: (u16, u16) = (15, 3);
    fn next_player_display(next_player: PlayerID) -> Result<(), P4Error> {
        let (x, y) = Self::NEXT_PLAYER_DISPLAY_POSITION;
        queue!(
            io::stdout(),
            SavePosition,
            PrintAt(x, y, "Au tour de "),
            DrawSquare(next_player.color()),
            RestorePosition
        )?;
        io::stdout().flush()?;
        Ok(())
    }
}

pub fn run() {
    let plt = Plate::default();
    let mut game = Game {
        plateau: plt,
        next_player: PlayerID::P1,
        player_one: Box::new(Human::new(PlayerID::P1)),
        player_two: Box::new(RBot),
    };
    //plateau.add_player(NumberCoord::new(1).expect("0").into(), player::PlayerID::P1).unwrap();
    //plateau.add_player(NumberCoord::new(2).expect("1"), player::PlayerID::P1).unwrap();
    //plateau.add_player(NumberCoord::new(3).expect("2"), player::PlayerID::P1).unwrap();
    //plateau.add_player(NumberCoord::new(7).expect("2"), player::PlayerID::P1).unwrap();
    //plateau.add_player(NumberCoord::new(12).expect("2"), player::PlayerID::P1).unwrap();
    ////plateau.add_player(NumberCoord::new(1).expect("3"), player::Player::P1);
    // //plateau.add_player(NumberCoord::new(1).expect("3.2"), player::Player::P1);

    // plateau.add_player(NumberCoord::new(5).expect("4"), player::Player::P1);
    // plateau.add_player(NumberCoord::new(5).expect("5"), player::Player::P1);
    // plateau.add_player(NumberCoord::new(5).expect("6"), player::Player::P1);
    // plateau.add_player(NumberCoord::new(5).expect("7"), player::Player::P2);

    // plateau.add_player(NumberCoord::new(8).expect("8"), player::Player::P2);

    // plateau.add_player(NumberCoord::new(9).expect("9"), player::Player::P2);
    // plateau.add_player(NumberCoord::new(9).expect("10"), player::Player::P2);
    // plateau.add_player(NumberCoord::new(15).expect("11"), player::Player::P1);
    execute!(io::stdout(), CreateTerminal).unwrap();

    game.run();

    thread::sleep(std::time::Duration::from_secs(3));
    execute!(io::stdout(), CloseTerminal).unwrap();
}

mod coord;
mod input;

mod error;
mod logic;
mod pilier;
mod plate;
mod player;
mod utils;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
