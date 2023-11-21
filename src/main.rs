use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Eq, Hash, Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Half,
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

fn value_in_cents(coin: &Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Half => 50,
    }
}

fn main() {
    let coins: [Coin; 5] = [Coin::Dime, Coin::Penny, Coin::Quarter, Coin::Nickel, Coin::Half];
    let amount: i32 = 22;
    let mut vector_table: Vec<Vec<HashMap<&Coin, i32>>> = Vec::new();

    for (coin_index, &ref coin) in coins.iter().enumerate() {
        vector_table.push(Vec::new());

        println!();

        for partial_amount in 1..amount + 1 {
            let count = partial_amount / value_in_cents(&coin);
            let remainder = partial_amount % value_in_cents(&coin);

            let mut proposal = HashMap::new();
            if count > 0 {
                proposal.insert(coin, count);
            }
            if coin_index == 0 {
                vector_table[coin_index as usize].push(proposal);
            } else if count == 0 {
                proposal.extend(&vector_table[coin_index-1][(remainder - 1) as usize]);
                vector_table[coin_index].push(proposal);
            }
            else {
                if remainder > 0 {
                    proposal.extend(&vector_table[coin_index][(remainder - 1) as usize]);
                }
                let proposal_coin_count : i32 = proposal.values().sum();

                let current_best = current_best(&vector_table, &coin_index, partial_amount);
                let current_best_coin_count: i32 = current_best.values().sum();

                if current_best_coin_count > 0 && proposal_coin_count >= current_best_coin_count {
                    vector_table[coin_index].push(current_best);
                } else {
                    vector_table[coin_index].push(proposal);
                }
            }
        }
    }
    println!("{:?}", vector_table[coins.len()-1][(amount-1) as usize]);
}

fn current_best<'a>(vector_table: &Vec<Vec<HashMap<&'a Coin, i32>>>, coin_index: &usize, partial_amount: i32) -> HashMap<&'a Coin, i32> {
    let mut current_best = vector_table[coin_index - 1][(partial_amount - 1) as usize].clone();
    let mut current_best_total = 0;
    for (coin, count) in current_best.iter() {
        current_best_total += value_in_cents(coin) * count
    }
    let current_best_remainder = partial_amount - current_best_total;
    if current_best_remainder > 0 && current_best_remainder != partial_amount {
        current_best.extend(&vector_table[*coin_index][(current_best_remainder - 1) as usize])
    }

    current_best
}
