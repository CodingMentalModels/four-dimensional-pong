use std::fmt::{Display, Formatter};


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Player {
    Blue,
    Red
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::Blue => write!(f, "Blue"),
            Player::Red => write!(f, "Red"),
        }
    }
}
