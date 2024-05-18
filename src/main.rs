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
    let base_hand = BaseHand::new(HandType::FlushFive, 20);
    let hand = vec![
        Card::new(2).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
        Card::new(2).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
        Card::new(2).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
        Card::new(2).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
        Card::new(2).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
    ];
    let jokers = vec![
        Joker::new(JokerClass::GenericPerTrigger(Modifiers::x_mult(2.0))),
        Joker::new(JokerClass::GenericPerTrigger(Modifiers::x_mult(2.0))),
        Joker::new(JokerClass::RetriggerAll),
        Joker::new(JokerClass::RetriggerAll),
    ];
    let unplayed_cards = vec![
        UnplayedCard::steel().add_seal(),
        UnplayedCard::steel().add_seal(),
        UnplayedCard::steel().add_seal(),
    ];

    let sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
    let score = sim.run();

    let score_truncated = score as u128;
    let num_digits = format!("{}", score_truncated).chars().count();
    let score_readable = score_truncated.separate_with_commas();
    let score_line = format!("Score: {} | Digits: {}", score_readable, num_digits);

    println!("{}", sim);
    println!("{}", "-".repeat(score_line.chars().count()));
    println!("{}", score_line);
}
