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
