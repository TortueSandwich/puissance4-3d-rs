use crate::coord::LetterCoord;

#[derive(Debug)]
pub enum P4Error {
    OutputInterfaceError(std::io::Error),
    OverFilledPillar(Option<LetterCoord>), //Box<dyn Coord>>),
    EmptyPlayerPlayed,
}

impl std::fmt::Display for P4Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutputInterfaceError(e) => {
                write!(f, "Error while wanting to use the terminal : {}", e)
            }
            Self::OverFilledPillar(Some(c)) => {
                write!(f, "Error at pillar {:?}, the pillar is overfilled", c)
            }
            Self::OverFilledPillar(None) => {
                write!(f, "Error at pillar ??, the pillar is overfilled")
            }
            Self::EmptyPlayerPlayed => write!(f, "An Void tried to play"),
        }
    }
}

impl std::error::Error for P4Error {}

impl From<std::io::Error> for P4Error {
    fn from(recived_e: std::io::Error) -> Self {
        Self::OutputInterfaceError(recived_e)
    }
}
