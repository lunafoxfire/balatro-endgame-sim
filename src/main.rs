#![allow(unused)]
mod sim;

use crate::sim::*;
use thousands::Separable;

fn main() {
    println!("Running...");

    let base_hand = BaseHand::new(HandType::FlushFive, 25);
    let hand = vec![
        Card::new(10).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
        Card::new(10).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
        Card::new(10).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
        Card::new(10).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
        Card::new(10).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
    ];
    let jokers = vec![
        Joker::new(JokerClass::RetriggerAll),
    ];
    let steel_cards_triggers = 0;
    let unplayed_card_triggers = 0;

    let mut sim = Simulation::init(base_hand, hand, jokers, steel_cards_triggers, unplayed_card_triggers);
    let score = sim.run();

    println!("Score: {}", (score as u128).separate_with_commas());
}
