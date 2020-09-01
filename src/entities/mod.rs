extern crate rand;

use std::fmt::{Display, Formatter, Result};

use rand::Rng;

pub struct Player {
    pub money: u32,
    pub name: String,
}

impl Player {
    pub fn roll_die() -> u32 {
        // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
        rand::thread_rng().gen_range(1, 7)
    }
    pub fn gain(&mut self, bet: u32) {
        self.money += bet;
    }
    pub fn loose(&mut self, bet: u32) {
        self.money -= bet;
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} has {}", self.name, self.money)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let player: Player = Player {
            money: 10,
            name: String::from("Foo"),
        };
        assert_eq!(10, player.money);
        assert_eq!("Foo", player.name);
    }

    #[test]
    fn gain() {
        let mut player: Player = Player {
            money: 10,
            name: String::from("Foo"),
        };
        player.gain(5);
        assert_eq!(15, player.money);
    }

    #[test]
    fn loose() {
        let mut player: Player = Player {
            money: 10,
            name: String::from("Foo"),
        };
        player.loose(5);
        assert_eq!(5, player.money);
    }
}

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