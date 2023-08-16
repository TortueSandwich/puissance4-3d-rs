use crate::coord::{Coord, Letter, LetterCoord, LetterCoordBuilder, NumberCoord};

#[derive(Clone, Default, PartialEq)]
pub struct Input {
    letter: Option<Letter>,
    number: Option<u8>,
}
impl Coord for Input {
    fn to_xy_coord(&self) -> (u16, u16) {
        let l = match self.letter.expect("need a letter to use") {
            Letter::A => 0,
            Letter::B => 1,
            Letter::C => 2,
            Letter::D => 3,
        };
        (l, self.number.expect("need a number to use") as u16)
    }
    fn to_num(&self) -> u8 {
        let (l, n) = self.to_xy_coord();
        l as u8 + n as u8 * 4
    }
}

impl Input {
    #[allow(dead_code)]
    fn reset(&mut self) {
        self.letter = None;
        self.number = None;
    }
    #[allow(dead_code)]
    fn give(&mut self) -> LetterCoord {
        let res = self.get_to_coord();
        self.reset();
        res
    }
    #[allow(dead_code)]
    fn get_to_coord(&self) -> LetterCoord {
        let letter = self.letter.expect("Not a letter");
        let number: u8 = self.number.expect("Not a usize number") as u8;
        let res = LetterCoordBuilder::default()
            .letter(letter)
            .number(number)
            .build()
            .expect("wtf here");
        res
    }
    pub fn set_letter(&mut self, letter: Letter) {
        self.letter = Some(letter);
    }
    pub fn set_number(&mut self, number: usize) {
        self.number = Some(number as u8);
    }
    pub fn is_valid(&self) -> bool {
        self.letter.is_some() && self.number.is_some()
    }

    pub fn split(&self) -> (Option<Letter>, Option<u8>) {
        (self.letter, self.number)
    }
}

impl std::fmt::Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (l, n) = self.split();
        let l = l.as_ref().map_or("?".to_owned(), |l| l.to_string());
        let n = n.as_ref().map_or("?".to_owned(), |n| n.to_string());
        write!(f, "{}{}", l, n)
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<(Letter, u8)> for Input {
    fn from(value: (Letter, u8)) -> Self {
        debug_assert!(value.1 < 4);
        let mut res = Input::default();
        res.set_letter(value.0);
        res.set_number(value.1.into());
        res
    }
}

impl From<u8> for Input {
    fn from(value: u8) -> Self {
        debug_assert!(value < 16);
        let res = NumberCoord::new(value);
        let inp = Input::from(res.to_ln_coord());
        inp
    }
}
