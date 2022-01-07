// use std::collections::{HashMap, VecDeque};
use pokemons_game::{Pokemon, BattleStates};
use std::io;
use rand::Rng;

// trait Pokemons {
//     fn n(&self);
// }

// impl Pokemons for Pokemon {
//     fn n(&self) {
//         println!("{}", self.name);
//     }
// }

// use crate::test::test1;
// mod test {
//     pub enum En {}
//
//     pub mod test1 {
//         pub fn ftest() -> i8 {
//             1
//         }
//     }
// }


// fn add(a: i32, b: i32, c: Option<i32>) -> i32 {
//     match c {
//         Option::None => a + b,
//         Option::Some(c) => a + b + c,
//     }
// }
//
// #[macro_export]
// macro_rules! add {
//     ($a: expr, $b: expr) => {
//         add($a, $b, Option::Some(5))
//     };
//     ($a: expr) => {
//       add($a, 1, Option::Some(2))
//     };
//     () => {
//         add(1, 2, Option::None)
//     };
// }

// N: number of pokemons in each team
const N: i32 = 1;
const MAX_POKEMON_ATK: i32 = 5;
const MAX_POKEMON_DF: i32 = 5;


fn main() {
    let mut player1 = Vec::new();
    let mut player2 = Vec::new();
    println!("Enter `1` or `2`");
    for _ in 0..N {
        let poks = gen_pokemons();
        println!("#1: Name: {} Atk: {} HP: {} Df: {}", poks[0].name, poks[0].atk, poks[0].hp, poks[0].df);
        println!("#2: Name: {} Atk: {} HP: {} Df: {}", poks[1].name, poks[1].atk, poks[1].hp, poks[1].df);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(..) => {
                let choice = input.trim().parse::<usize>().unwrap();
                println!("1st player's choice ->{}", input);
                if choice > 2 || choice < 0 {
                    panic!("ERROR: specify `1` or `2`")
                }
                player1.push(poks[choice - 1].clone());
            }
            Err(e) => panic!("ERROR: {}", e),
        }

        let poks = gen_pokemons();
        println!("#1: Name: {} Atk: {} HP: {} Df: {}", poks[0].name, poks[0].atk, poks[0].hp, poks[0].df);
        println!("#2: Name: {} Atk: {} HP: {} Df: {}", poks[1].name, poks[1].atk, poks[1].hp, poks[1].df);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(..) => {
                let choice = input.trim().parse::<usize>().unwrap();
                println!("2nd player's choice ->{}", input);
                if choice > 2 || choice < 0 {
                    panic!("ERROR: specify `1` or `2`")
                }
                player2.push(poks[choice - 1].clone());
                // let choice = input.trim().parse::<i32>();
                // match choice {
                //     Ok(..) => {
                //         println!("2nd player's choice ->{}", input);
                //         player2.push(choice.unwrap());
                //     }
                //     Err(e) => panic!("ERROR: not a number ({})", e),
                // }
            }
            Err(e) => panic!("ERROR: {}", e),
        }
    }
    // dbg!(player1, player2);

    let mut turn = 1;
    let mut shift = 0;
    for pok1 in 0..player1.len() {
        if player1[pok1 + shift].hp <= 0 { continue; };
        for pok2 in 0..player2.len() {
            if player2[pok2].hp <= 0 {
                continue;
            };
            if player1[pok1 + shift].hp <= 0 {
                shift + 1;
            };
            loop {
                if player2[pok2].hp <= 0 {
                    break;
                };
                if player1[pok1 + shift].hp <= 0 {
                    shift + 1;
                    break;
                };
                if turn == 1 {
                    player1[pok1 + shift].attack(&mut player2[pok2]);
                    turn = 2;
                } else {
                    player2[pok2].attack(&mut player1[pok1 + shift]);
                    turn = 1;
                }
            }
        }
    }
    // dbg!(&player1, &player2);

    let mut win = 0;
    for pok in &player1 {
        if pok.hp > 0 {
            win = 1;
            break;
        }
    }
    for pok in &player2 {
        if pok.hp > 0 {
            win = 2;
            break;
        }
    }
    if win == 1 {
        println!("Congrats to player 1! You won!")
    } else if win == 2 {
        println!("Congrats to player 2! You won!")
    } else {
        println!("I am confused... No one won!")
    }
}

fn gen_pokemons() -> Vec<Pokemon> {
    vec![
        Pokemon {
            name: String::from("Pikachu"),
            hp: 100,
            atk: rand::thread_rng().gen_range(1..=MAX_POKEMON_ATK),
            df: rand::thread_rng().gen_range(1..=MAX_POKEMON_DF),
        },
        Pokemon {
            name: String::from("Charizard"),
            hp: 100,
            atk: rand::thread_rng().gen_range(1..=MAX_POKEMON_ATK),
            df: rand::thread_rng().gen_range(1..=MAX_POKEMON_DF),
        },
    ]
}
