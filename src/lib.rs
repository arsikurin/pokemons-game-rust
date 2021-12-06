#[derive(Debug)]
pub enum BattleStates {
    NotStarted = 0,
    Started = 1,
    Finished = 2,
}

#[derive(Debug, Clone)]
pub struct Pokemon {
    pub name: String,
    pub hp: i32,
    pub atk: i32,
    pub df: i32,
}

// trait Clone {
//     fn clone(&self) -> Self;
// }

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
