#![allow(unused)]
mod sim {
    mod sim; pub use sim::*;
    mod modifiers; pub use modifiers::*;
    mod jokers; pub use jokers::*;
    mod cards; pub use cards::*;
}

use crate::sim::*;
use thousands::Separable;

fn main() {
    println!("Running...");

    let base_hand = BaseHand::new(HandType::FlushFive, 25);
    let hand = vec![
        Card::new(10).add_enhancement(Enhancement::Glass).add_edition(Edition::Poly).add_seal(Seal::Red),
        Card::new(10).add_enhancement(Enhancement::Glass).add_edition(Edition::Poly).add_seal(Seal::Red),
        Card::new(10).add_enhancement(Enhancement::Glass).add_edition(Edition::Poly).add_seal(Seal::Red),
        Card::new(10).add_enhancement(Enhancement::Glass).add_edition(Edition::Poly).add_seal(Seal::Red),
        Card::new(10).add_enhancement(Enhancement::Glass).add_edition(Edition::Poly).add_seal(Seal::Red),
    ];
    let jokers = vec![
        Joker::new(JokerClass::RetriggerAll),
        Joker::new(JokerClass::RetriggerAll),
        Joker::new(JokerClass::RetriggerAll),
        Joker::new(JokerClass::RetriggerAll),
        Joker::new(JokerClass::RetriggerAll),
    ];
    let unplayed_cards = vec![

    ];

    let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
    let score = sim.run();

    let score_truncated = score as u128;
    let num_digits = format!("{}", score_truncated).chars().count();
    let score_readable = score_truncated.separate_with_commas();
    println!("Score: {} | Digits: {}", score_readable, num_digits);
}
