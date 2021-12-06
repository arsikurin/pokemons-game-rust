// use std::collections::{HashMap, VecDeque};
use pokemons_game::{Pokemon, PokemonStates, TrainerStates, BattleStates};
use std::io;

// trait Pokemons {
//     fn n(&self);
// }

// impl Pokemons for Pokemon {
//     fn n(&self) {
//         println!("{}", self.name);
//     }
// }

// use crate::test::test1;
//
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

fn main() {
    let mut pokemon1 = Pokemon { state: PokemonStates::Caught, name: String::from("pikachu"), hp: 100, atk: 10, df: 0 };
    let mut pokemon2 = Pokemon { state: PokemonStates::Caught, name: String::from("kurkur"), hp: 100, atk: 50, df: 0 };
    // println!("{:?}\n{:?}\n\n", pokemon1, pokemon2);
    // pokemon1.attack(&mut pokemon2);
    // println!("{:?}\n{:?}", pokemon1, pokemon2);

    println!("#1: Name: {} Atk: {} HP: {} Df: {}", pokemon1.name, pokemon1.atk, pokemon1.hp, pokemon1.df);
    println!("#2: Name: {} Atk: {} HP: {} Df: {}", pokemon2.name, pokemon2.atk, pokemon2.hp, pokemon2.df);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => {
            match &input.trim().parse::<i32>() {
                Ok(..) => println!("1nd player's choice ->{}", input),
                Err(e) => println!("ERROR: not a number ({})", e),
            }
        }
        Err(error) => println!("ERROR: {}", error),
    }

    println!("#1: Name: {} Atk: {} HP: {} Df: {}", pokemon1.name, pokemon1.atk, pokemon1.hp, pokemon1.df);
    println!("#2: Name: {} Atk: {} HP: {} Df: {}", pokemon2.name, pokemon2.atk, pokemon2.hp, pokemon2.df);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => {
            match &input.trim().parse::<i32>() {
                Ok(..) => println!("2nd player's choice ->{}", input),
                Err(e) => println!("ERROR: not a number ({})", e),
            }
        }
        Err(error) => println!("ERROR: {}", error),
    }

}
