use crate::functions::{get_int_from_input, get_string_from_input};
use crate::entities::Player;

mod functions;
mod entities;

fn main() {
    println!("Enter croupier's name:");
    let mut croupier: Player = Player { money: 10, name: get_string_from_input() };

    println!("Enter your name:");
    let mut you = Player { money: 10, name: get_string_from_input() };

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