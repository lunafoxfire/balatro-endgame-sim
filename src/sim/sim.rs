#[cfg(test)]
#[path ="sim.test.rs"]
mod test;
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
    steel_card_triggers: u32,
    unplayed_card_triggers: u32, // kings for Baron
    score: Score,
    retriggers: Retriggers,
}

impl Simulation {
    pub fn init(base_hand: BaseHand, hand: Vec<Card>, jokers: Vec<Joker>, steel_card_triggers: u32, unplayed_card_triggers: u32) -> Self {
        Self {
            base_hand,
            hand,
            jokers,
            steel_card_triggers,
            unplayed_card_triggers,
            score: Score::default(),
            retriggers: Retriggers::default(),
        }
    }

    pub fn run(&mut self) -> f64 {
        let base_score = self.base_hand.score();
        self.score.chips = base_score.chips as f64;
        self.score.mult = base_score.mult as f64;

        for joker in self.jokers.iter() {
            joker.on_setup(&mut self.retriggers);
        };

        for (i, card) in self.hand.iter().enumerate() {
            let mut triggers = 1;
            triggers += self.retriggers.all;
            if i == 0 {
                triggers += self.retriggers.first;
            }
            if *card.seal() == Seal::Red {
                triggers += 1;
            }

            for _ in 0..triggers {
                card.on_trigger(&mut self.score);
                for joker in self.jokers.iter() {
                    joker.on_card_trigger(&mut self.score, i);
                }
            }
        };

        for _ in 0..self.unplayed_card_triggers {
            let mut triggers = 1;
            triggers += self.retriggers.hand;
            for _ in 0..triggers {
                for joker in self.jokers.iter() {
                    joker.on_unplayed_trigger(&mut self.score);
                }
            }
        }

        for _ in 0..self.steel_card_triggers {
            let mut triggers = 1;
            triggers += self.retriggers.hand;
            for _ in 0..triggers {
                self.score.mult *= 1.5;
            }
        };

        for joker in self.jokers.iter() {
            joker.on_trigger(&mut self.score);
        };

        self.score.total()
    }
}
