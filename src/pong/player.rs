use std::fmt::{Display, Formatter};
use bevy::prelude::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Target {
    position: Vec3,
}

impl Target {

    pub fn new(position: Vec3) -> Self {
        Self {
            position,
        }
    }
}

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
