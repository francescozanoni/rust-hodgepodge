use std::fmt::{Display, Formatter, Result};

use crate::entities::Player;

pub struct Game {
    pub player_1: Player,
    pub player_2: Player,
}

impl Game {
    pub fn is_finished(&self) -> bool {
        !(self.player_1.money > 0 && self.player_2.money > 0)
    }
    pub fn get_winner(&self) -> &Player {
        if self.player_1.money > 0 {
            &self.player_1
        } else {
            &self.player_2
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\nSituation: {}, {}\n", self.player_1, self.player_2)
    }
}