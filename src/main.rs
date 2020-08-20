use crate::entities::game::Game;
use crate::entities::Player;
use crate::functions::{get_int_from_input, get_string_from_input};

mod entities;
mod functions;

const START_MONEY: u32 = 10;

fn main() {
    println!("Enter player 1's name:");
    let player_1: Player = Player {
        money: START_MONEY,
        name: get_string_from_input(),
    };

    println!("Enter player 2's name:");
    let player_2 = Player {
        money: START_MONEY,
        name: get_string_from_input(),
    };

    let mut game = Game { player_1, player_2 };

    println!("Situation: {}, {}\n", game.player_1, game.player_2);

    while game.is_finished() == false {
        println!("Enter money to bet:");
        let bet = get_int_from_input();

        let player_1_die = Player::roll_die();
        println!("{}'s die: {}", game.player_1.name, player_1_die);

        let player_2_die = Player::roll_die();
        println!("{}'s die: {}", game.player_2.name, player_2_die);

        if player_1_die > player_2_die {
            game.player_2.loose(bet);
            game.player_1.gain(bet);
        } else if player_2_die > player_1_die {
            game.player_2.gain(bet);
            game.player_1.loose(bet);
        }

        println!("Situation: {}, {}\n", game.player_1, game.player_2);
    }

    println!("{} wins!\n", game.get_winner().name);
}
