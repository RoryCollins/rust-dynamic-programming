use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

pub trait Valuable {
    fn get_value(&self) -> u16;
}

#[derive(Eq, Hash, Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Half,
}

impl Valuable for Coin {
    fn get_value(&self) -> u16 {
    match self {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Half => 50,
    }
}
}

impl PartialEq<Self> for Coin {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Coin::Penny, &Coin::Penny) => true,
            _ => false
        }
    }
}

impl Display for Coin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Coin::Penny => write!(f, "penny"),
            Coin::Nickel => write!(f, "nickel"),
            Coin::Dime => write!(f, "dime"),
            Coin::Quarter => write!(f, "quarter"),
            Coin::Half => write!(f, "half"),
        }
    }
}

fn main() {
    let coins: [Coin; 5] = [Coin::Dime, Coin::Penny, Coin::Quarter, Coin::Nickel, Coin::Half];
    let amount = 22;
    let foo = find_best(&coins, amount);
    println!("{:?}", foo);
}

fn find_best<T>(items: &[T; 5], target: u16) -> HashMap<&T, u16>
    where T : Valuable + Eq + Hash
{
    let mut vector_table: Vec<Vec<HashMap<&T, u16>>> = Vec::new();
    for (index, item) in (&items).iter().enumerate() {
        vector_table.push(Vec::new());
        for partial_amount in 1..target + 1 {
            let count = partial_amount / item.get_value();
            let remainder = partial_amount % item.get_value();

            let mut proposal = HashMap::new();
            if count > 0 {
                proposal.insert(item, count);
            }
            if index == 0 {
                vector_table[index].push(proposal);
            } else if count == 0 {
                proposal.extend(&vector_table[index - 1][usize::from(remainder - 1)]);
                vector_table[index].push(proposal);
            } else {
                if remainder > 0 {
                    proposal.extend(&vector_table[index][usize::from(remainder - 1)]);
                }
                let proposal_coin_count: u16 = proposal.values().sum();

                let current_best = current_best(&vector_table, &index, partial_amount);
                let current_best_coin_count = current_best.values().sum();

                if current_best_coin_count > 0 && proposal_coin_count >= current_best_coin_count {
                    vector_table[index].push(current_best);
                } else {
                    vector_table[index].push(proposal);
                }
            }
        }
    }
    vector_table[(items).len()-1][usize::from(target - 1)].clone()
}

fn current_best<'a, T>(vector_table: &Vec<Vec<HashMap<&'a T, u16>>>, index: &usize, partial_amount: u16) -> HashMap<&'a T, u16>
where T : Valuable + Eq + Hash
{
    let mut current_best = vector_table[index - 1][usize::from(partial_amount - 1)].clone();
    let mut current_best_total = 0;
    for (coin, count) in current_best.iter() {
        current_best_total += coin.get_value() * count
    }
    let current_best_remainder = partial_amount - current_best_total;
    if current_best_remainder > 0 && current_best_remainder != partial_amount {
        current_best.extend(&vector_table[*index][usize::from(current_best_remainder - 1)])
    }

    current_best
}
