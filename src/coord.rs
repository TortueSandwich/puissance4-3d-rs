use std::str::FromStr;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Letter {
    A,
    B,
    C,
    D,
}

pub trait Coord: std::fmt::Debug {
    fn to_num(&self) -> u8;
    fn to_xy_coord(&self) -> (u16, u16) {
        let n = self.to_num() as u16;
        (n % 4, n / 4)
    }
    fn to_ln_coord(&self) -> (Letter, u8) {
        let n = self.to_num();
        let l = match n % 4 {
            0 => Letter::A,
            1 => Letter::B,
            2 => Letter::C,
            3 => Letter::D,
            _ => panic!("Wtf donne moi un bon nb"),
        };
        (l, n / 4)
    }

    fn as_letter_coord(&self) -> LetterCoord {
        LetterCoord::from(self.to_ln_coord())
    }
    fn as_number_coord(&self) -> NumberCoord {
        NumberCoord::from(self.to_ln_coord())
    }
}

// impl std::fmt::Debug for dyn Coord {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let (l,n) = self.to_ln_coord();
//         write!(f, "{}{}",l,n)
//     }
// }

#[derive(derive_builder::Builder, Clone, PartialEq)]
pub struct LetterCoord {
    letter: Letter,

    #[builder(setter(into))]
    number: u8,
}

#[derive(Clone, PartialEq)]
pub struct NumberCoord {
    n: u8,
}

impl Coord for LetterCoord {
    fn to_num(&self) -> u8 {
        let (l, n) = self.to_xy_coord();
        l as u8 + n as u8 * 4
    }
    fn to_xy_coord(&self) -> (u16, u16) {
        let l: u16 = match self.letter {
            Letter::A => 0,
            Letter::B => 1,
            Letter::C => 2,
            Letter::D => 3,
        };
        (l, self.number as u16)
    }
}

impl Coord for NumberCoord {
    fn to_num(&self) -> u8 {
        self.n as u8
    }
}

impl NumberCoord {
    pub fn new(n: u8) -> NumberCoord {
        debug_assert!(n < 16);
        NumberCoord { n }
    }
}

impl std::fmt::Debug for NumberCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (l, n) = self.to_ln_coord();
        write!(f, "{}{}", l, n)?;
        Ok(())
    }
}
impl std::fmt::Debug for LetterCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.letter, self.number)
    }
}

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Letter::A => "A",
                Letter::B => "B",
                Letter::C => "C",
                Letter::D => "D",
            }
        )
    }
}

impl From<(Letter, u8)> for NumberCoord {
    fn from(value: (Letter, u8)) -> Self {
        debug_assert!(value.1 < 4);
        let li32 = match value.0 {
            Letter::A => 0,
            Letter::B => 1,
            Letter::C => 2,
            Letter::D => 3,
        };
        NumberCoord::new(value.1 * 4 + li32)
    }
}

impl From<(Letter, u8)> for LetterCoord {
    fn from(value: (Letter, u8)) -> Self {
        debug_assert!(value.1 < 4);
        LetterCoordBuilder::default()
            .letter(value.0)
            .number(value.1)
            .build()
            .expect(
                format!(
                    "Impossible to build a Letter coord from {}{}",
                    value.0, value.1
                )
                .as_str(),
            )
    }
}

impl From<u8> for Letter {
    fn from(value: u8) -> Self {
        match value {
            0 => Letter::A,
            1 => Letter::B,
            2 => Letter::C,
            3 => Letter::D,
            _ => unreachable!("Conversion u8 -> Letter :\nvalue : {value}"),
        }
    }
}

impl FromStr for Letter {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Letter::A),
            "B" => Ok(Letter::B),
            "C" => Ok(Letter::C),
            "D" => Ok(Letter::D),
            _ => Err(()),
        }
    }
}
