#[cfg(test)]
#[path ="sim.test.rs"]
mod test;
use std::fmt::Display;
use crate::sim::*;

#[derive(Default, Clone, Copy)]
pub struct Score {
    pub chips: f64,
    pub mult: f64,
}

impl Score {
    pub fn apply(&mut self, modifiers: &Modifiers) {
        self.chips += modifiers.chips as f64;
        self.mult += modifiers.mult as f64;
        self.mult *= modifiers.x_mult as f64;
    }

    pub fn total(&self) -> f64 {
        self.chips * self.mult
    }
}

#[derive(Default, Clone, Copy)]
pub struct Retriggers {
    pub all: u32,
    pub first: u32,
    pub hand: u32,
}

pub struct Simulation {
    base_hand: BaseHand,
    hand: Vec<Card>,
    jokers: Vec<Joker>,
    unplayed_cards: Vec<UnplayedCard>,
}

impl Simulation {
    pub fn init(base_hand: BaseHand, hand: Vec<Card>, jokers: Vec<Joker>, unplayed_cards: Vec<UnplayedCard>) -> Self {
        Self {
            base_hand,
            hand,
            jokers,
            unplayed_cards,
        }
    }

    pub fn run(&self) -> f64 {
        let mut score = Score::default();
        let mut retriggers = Retriggers::default();

        let base_score = self.base_hand.score();
        score.chips = base_score.chips as f64;
        score.mult = base_score.mult as f64;

        for joker in self.jokers.iter() {
            joker.on_setup(&mut retriggers);
        };

        for (i, card) in self.hand.iter().enumerate() {
            let mut triggers = 1;
            triggers += retriggers.all;
            if i == 0 {
                triggers += retriggers.first;
            }
            if *card.seal() == Seal::Red {
                triggers += 1;
            }

            for _ in 0..triggers {
                card.on_trigger(&mut score);
                for joker in self.jokers.iter() {
                    joker.on_card_trigger(&mut score, i);
                }
            }
        };

        for card in self.unplayed_cards.iter() {
            let mut triggers = 1;
            triggers += retriggers.hand;
            if card.has_red_seal {
                triggers += 1;
            }
            for _ in 0..triggers {
                if card.is_steel {
                    score.mult *= 1.5;
                }
                for joker in self.jokers.iter() {
                    joker.on_unplayed_trigger(&mut score, &card);
                }
            }
        }

        for joker in self.jokers.iter() {
            joker.on_trigger(&mut score);
        };

        score.total()
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output += &format!("Base: {}", self.base_hand);
        output += &String::from("\nHand:");
        for card in self.hand.iter() {
            output += &format!("\n    {}", card);
        }
        output += &String::from("\nJokers:");
        for joker in self.jokers.iter() {
            output += &format!("\n    {}", joker);
        }
        output += &format!("\nUnplayed ({}):", self.unplayed_cards.len());
        for card in self.unplayed_cards.iter() {
            output += &format!("\n    {}", card);
        }
        write!(f, "{}", output)
    }
}
