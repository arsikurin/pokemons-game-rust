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

// number of pokemons in each team
const N: i32 = 2; // nmref
const MAX_POKEMON_ATK: i32 = 5;
const MAX_POKEMON_DF: i32 = 5;


fn main() {
    let mut player1: Vec<Pokemon> = Vec::new();
    let mut player2 = Vec::new();

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
}

fn gen_pokemons() -> Vec<Pokemon> {
    vec![
        Pokemon {
            name: String::from("pikachu"),
            hp: 100,
            atk: rand::thread_rng().gen_range(1..=MAX_POKEMON_ATK),
            df: rand::thread_rng().gen_range(1..=MAX_POKEMON_DF),
        },
        Pokemon {
            name: String::from("kurkur"),
            hp: 100,
            atk: rand::thread_rng().gen_range(1..=MAX_POKEMON_ATK),
            df: rand::thread_rng().gen_range(1..=MAX_POKEMON_DF),
        },
    ]
}
