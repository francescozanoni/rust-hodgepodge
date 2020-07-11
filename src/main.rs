extern crate rand;

use rand::Rng;
use std::fmt::{Display, Formatter, Result};

fn get_string_from_input() -> String {
    // https://www.tutorialspoint.com/rust/rust_input_output.htm
    let mut a_string = String::new();
    std::io::stdin().read_line(&mut a_string).unwrap();
    String::from(a_string.trim())
}

fn get_int_from_input() -> u32 {
    // https://www.programming-idioms.org/idiom/120/read-integer-from-stdin/1906/rust
    get_string_from_input().parse().unwrap()
}

struct Player {
    money: u32,
    name: String
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

fn main() {
    
    println!("Enter croupier's name:");
    let mut croupier = Player{money: 10, name: get_string_from_input()};
    
    println!("Enter your name:");
    let mut you = Player{money: 10, name: get_string_from_input()};
    
    println!("\nSituation: {}, {}\n", croupier, you);
    
    while croupier.money > 0 && you.money > 0 {
    
        println!("Enter money to bet:");
        
        let bet = get_int_from_input();
        
        let croupier_die = Player::roll_die();
        let your_die = Player::roll_die();
        
        println!("{}'s die: {}", croupier.name, croupier_die);
        println!("{}'s die: {}", you.name, your_die);
        
        if croupier_die > your_die {
            you.loose(bet);
            croupier.gain(bet);
        } else if your_die > croupier_die {
            you.gain(bet);
            croupier.loose(bet);
        }
        
        println!("Situation: {}, {}\n", croupier, you);
   
    }
    
    if croupier.money == 0 {
        println!("{} wins!\n", you.name);
    } else {
        println!("{} wins!\n", croupier.name);
    }
    
}