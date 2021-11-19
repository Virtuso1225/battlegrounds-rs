extern crate rand;

use rand::Rng;

struct Minions {
    total_num: u64,
}

struct Player {
    minions: Minions,
    turn: bool,
    health: u64,
    shield: u64,
}

//temporary Player status initilization
fn battle_player_init(minions: Minions) -> Player {
    Player {
        minions,
        turn: false,
        health: 40,
        shield: 0,
    }
}

fn minions_init(num: u64) -> Minions {
    Minions { total_num: num }
}
//Battle initilization
fn battle_init() {
    let player_one_minions = minions_init(5);
    let player_two_minions = minions_init(5);

    let mut player_one = battle_player_init(player_one_minions);
    let mut player_two = battle_player_init(player_two_minions);

    let player_one_minions_tot = player_one.minions.total_num;
    let player_two_minions_tot = player_two.minions.total_num;

    if player_one_minions_tot > player_two_minions_tot {
        player_one.turn = true;
        println!("it is Player One's turn");
    } else if player_one_minions_tot < player_two_minions_tot {
        player_two.turn = true;
        println!("it is Player Two's turn");
    } else {
        //player's turn set by random
        let turn = rand::thread_rng().gen_range(0, 2);
        if turn == 0 {
            player_one.turn = true;
            println!("it is Player One's turn by random");
        } else {
            player_two.turn = true;
            println!("it is Player Two's turn by random");
        }
    }
}

fn main() {
    battle_init();
}
