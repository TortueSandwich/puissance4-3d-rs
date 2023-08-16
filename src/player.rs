use crate::{error::P4Error, utils::DrawSquare};
use crossterm::{style::Color, Command};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerID {
    P1,
    P2,
    Empty,
}

impl Default for PlayerID {
    fn default() -> Self {
        PlayerID::Empty
    }
}

impl PlayerID {
    const VOID_COLOR: Color = Color::AnsiValue(236);
    const PLAYER1_COLOR: Color = Color::AnsiValue(223);
    const PLAYER2_COLOR: Color = Color::AnsiValue(130);

    pub fn color(&self) -> Color {
        match self {
            PlayerID::Empty => Self::VOID_COLOR,
            PlayerID::P1 => Self::PLAYER1_COLOR,
            PlayerID::P2 => Self::PLAYER2_COLOR,
        }
    }
    pub fn joue(&mut self) {
        *self = match self {
            PlayerID::Empty => panic!("{:?}", P4Error::EmptyPlayerPlayed),
            PlayerID::P1 => PlayerID::P2,
            PlayerID::P2 => PlayerID::P1,
        }
    }
}
impl std::fmt::Display for PlayerID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DrawSquare(self.color()).write_ansi(f)?;
        Ok(())
    }
}
