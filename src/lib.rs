#[derive(Debug)]
pub enum PokemonStates {
    Caught = 0,
    Wild = 1,
}

#[derive(Debug)]
pub enum TrainerStates {
    Idle = 0,
    Fighting = 1,
}

#[derive(Debug)]
pub enum BattleStates {
    NotStarted = 0,
    Started = 1,
    Finished = 2,
}

#[derive(Debug)]
pub struct Pokemon {
    pub state: PokemonStates,
    pub name: String,
    pub hp: i32,
    pub atk: i32,
    pub df: i32,
}

impl Pokemon {
    pub fn attack(&mut self, other: &mut Pokemon) {
        let mut damage = self.atk;
        if other.hp <= 0 {
            other.hp = 0;
            return;
        }
        if self.hp <= 0 {
            self.hp = 0;
            return;
        }
        if damage == 0 { damage = self.atk - other.df; }
        if damage > 0 {
            other.hp -= damage;
        } else {
            other.hp -= 1
        }
    }
}
