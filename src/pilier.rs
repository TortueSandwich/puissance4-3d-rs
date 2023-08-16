use std::io::{stdout, Write};

use crate::coord::{Coord, Letter};
use crate::error::P4Error;
use crate::player::PlayerID;
use crate::utils::{DrawSquareAt, DrawSquareAtRawCoord, PrintAt};
use crossterm::queue;

#[derive(Clone, Copy, Debug)]
pub struct Pillar {
    pub data: [PlayerID; 4],
    height: u8,
}

pub struct Plane {
    pub data: [PlayerID; 16],
}

impl std::fmt::Display for Pillar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for p in self.data.into_iter() {
            if p == PlayerID::Empty {
                write!(f, "v,")?;
            } else {
                write!(f, "{:?},", p)?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Default for Pillar {
    fn default() -> Self {
        Pillar {
            data: [PlayerID::Empty; 4],
            height: 0,
        }
    }
}

impl Pillar {
    #[allow(dead_code)]
    fn new(v: Vec<PlayerID>) -> Pillar {
        debug_assert!(v.len() == 4);
        let temp: [PlayerID; 4] = v.try_into().unwrap();
        Pillar {
            data: temp,
            height: 0,
        }
    }
    #[allow(dead_code)]
    pub fn is_full(&self) -> bool {
        self.height >= 4
    }

    fn validate(&self) {
        assert!(
            self.height < 5,
            "Hauteur trop haute !!!\n antendu : < 5, recu : {}",
            self.height
        );
        assert!(
            self.data[self.height as usize..]
                .into_iter()
                .all(|p| { p == &PlayerID::Empty }),
            "Il y a un vide entre deux pion !\n{:?}",
            self.data
        );
    }

    pub fn add_player(&mut self, p: PlayerID) -> Result<(), P4Error> {
        debug_assert!(p != PlayerID::Empty);
        if self.height >= 4 {
            return Err(P4Error::OverFilledPillar(None));
        }
        self.data[self.height as usize] = p;
        self.height += 1;
        self.validate();
        Ok(())
    }

    pub fn get_height(&self) -> u8 {
        self.height.clone()
    }

    pub fn get_player(&self, i: usize) -> PlayerID {
        self.data[i].clone()
    }

    pub fn draw(&self, origin: (u16, u16), pillarname: impl Coord) -> Result<(), P4Error> {
        let (l, n) = pillarname.to_ln_coord();
        queue!(
            std::io::stdout(),
            PrintAt(origin.0, origin.1, format!("{}{}", l, n).as_str())
        )
        .map_err(|e| P4Error::OutputInterfaceError(e))?;
        for (i, d) in self.data.iter().rev().enumerate() {
            let coord = (0, i as u16);
            let x = origin.0 + coord.0;
            let y = origin.1 + coord.1 + 1;
            queue!(std::io::stdout(), DrawSquareAtRawCoord(x, y, d.color()))?;
        }
        stdout()
            .flush()
            .map_err(|e| P4Error::OutputInterfaceError(e))?;
        Ok(())
    }
}

impl Plane {
    pub fn draw(&self, origin: (u16, u16)) -> Result<(), P4Error> {
        for i in 0..4 {
            let u = i + 1;
            queue!(
                std::io::stdout(),
                PrintAt(origin.0, u + origin.1, format!("{}", i).as_str()),
                PrintAt(
                    2 * (1 + i) + origin.0 * 2,
                    origin.1,
                    format!(" {}", Letter::from(i as u8)).as_str()
                ),
            )
            .map_err(|e| P4Error::OutputInterfaceError(e))?;
        }
        for (i, d) in self.data.iter().enumerate() {
            let coord = (i as u16 % 4, i as u16 / 4);
            let x = origin.0 + coord.0 + 1;
            let y = origin.1 + coord.1 + 1;
            queue!(std::io::stdout(), DrawSquareAt(x, y, d.color()))
                .map_err(|e| P4Error::OutputInterfaceError(e))?;
        }
        Ok(())
    }
}
