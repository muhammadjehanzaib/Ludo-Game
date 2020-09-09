use rand::Rng;
use std::collections::HashMap;
use std::io;
use std::process;
fn no_of_players() -> u32 {
    let no_of_players: u32 = loop {
        println!("Enter Number of Players!");

        let mut no_of_players = String::new();

        io::stdin()
            .read_line(&mut no_of_players)
            .expect("Failed to read line");

        let no_of_players: u32 = match no_of_players.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only Number Allowed, Try Again");
                continue;
            }
        };
        break no_of_players;
    };
    no_of_players
}

fn main() {
    println!("Welcome To LUDO");
    let no_of_players = no_of_players();

    let mut name_players: Vec<String> = Vec::new();

    for i in 1..no_of_players + 1 {
        println!("Enter the Name of Player {}", i);
        let mut names = String::new();
        io::stdin()
            .read_line(&mut names)
            .expect("Failed to read line");
        let names = names.trim().parse().expect("Some thing is worng");
        name_players.push(names);
    }
    let mut ludo = HashMap::new();
    for i in name_players.iter() {
        ludo.insert(i, 0);
    }
    println!("{:?}", name_players);
    println!("{:?}", ludo);

    let mut turn: u32 = 1;

    loop {
        let mut current_dice = 0;

        for players in 0..no_of_players {
            let dice = rand::thread_rng().gen_range(1, 7);

            if dice == 6 {
                current_dice = current_dice + dice;
                let dice = rand::thread_rng().gen_range(1, 7);
                if dice == 6 {
                    current_dice = current_dice + dice;
                    let dice = rand::thread_rng().gen_range(1, 7);
                    if dice == 6 {
                        current_dice = 0;
                    } else {
                        current_dice = current_dice + dice;
                    }
                } else {
                    current_dice = current_dice + dice;
                }
            } else {
                current_dice = current_dice + dice;
            }

            let pre_dice_score = match ludo.get(&name_players[players as usize]) {
                Some(expr) => expr,
                None => &0,
            };
            if pre_dice_score + current_dice == 100 {
                println!(
                    "Turn {} Dice Roll of Player {} -    {:?} is      {}    amd Total is      {}",
                    turn,
                    players + 1,
                    &name_players[players as usize],
                    current_dice,
                    pre_dice_score + dice
                );

                println!(
                    "Congratulation! Player {:?} has won on trun {}",
                    &name_players[players as usize], turn
                );
                process::exit(0);
            } else if pre_dice_score + current_dice > 100 {
                current_dice = 0;
                let total = pre_dice_score + 0;
                ludo.insert(&name_players[players as usize], total);
            } else {
                let total = pre_dice_score + current_dice;
                ludo.insert(&name_players[players as usize], total);
            }

            let publish_pre_dice_score = match ludo.get(&name_players[players as usize]) {
                Some(expr) => expr,
                None => &0,
            };
            println!(
                "Turn  {}   Dice Roll of Player {} - {:?}  is {} and Total is  {}",
                turn,
                players + 1,
                &name_players[players as usize],
                current_dice,
                publish_pre_dice_score
            );

            current_dice = 0;
        }
        for players in 0..no_of_players {
            for j in players + 1..no_of_players {
                if ludo.get(&name_players[players as usize]) == ludo.get(&name_players[j as usize])
                {
                    println!(
                        "        \n   Alas!     {:?}      Kicked by      {:?}",
                        &name_players[players as usize], &name_players[j as usize]
                    );
                    ludo.insert(&name_players[players as usize], 0);
                }
            }
        }

        turn += 1;
        let mut enter = String::new();
        io::stdin()
            .read_line(&mut enter)
            .expect("Something is worng");
    }
}