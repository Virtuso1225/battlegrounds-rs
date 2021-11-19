struct Minions {
    totalNum: u64,
}

struct Player {
    minions: Minions,
    turn: bool,
}

//temporary Player status initilization
fn BattlePlayerInit(minions: Minions) -> Player {
    Player {
        minions,
        turn: false,
    }
}

fn MinionsInit(num: u64) -> Minions {
    Minions { totalNum: num }
}
//Battle initilization
fn BattleInit() {
    let PlayerOneMinions = MinionsInit(5);
    let PlayerTwoMinions = MinionsInit(3);

    let mut PlayerOne = BattlePlayerInit(PlayerOneMinions);
    let mut PlayerTwo = BattlePlayerInit(PlayerTwoMinions);

    if (PlayerOne.minions > PlayerTwo.minions) {
        PlayerOne.turn = true;
        println!("it is Player One's turn");
    } else if (PlayerOne.minions < PlayerTwo.minions) {
        PlayerOne.turn = true;
        println!("it is Player Two's turn");
    } else {
        //random
    }
}
