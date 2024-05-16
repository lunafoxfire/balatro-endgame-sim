use crate::sim::*;

pub struct Card {
    value: u32,
    enhancement: Enhancement,
    edition: Edition,
    seal: Seal,
}

impl Card {
    pub fn value(&self) -> &u32 { &self.value }
    pub fn enhancement(&self) -> &Enhancement { &self.enhancement }
    pub fn edition(&self) -> &Edition { &self.edition }
    pub fn seal(&self) -> &Seal { &self.seal }
}

impl Card {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            enhancement: Enhancement::None,
            edition: Edition::None,
            seal: Seal::None,
        }
    }

    pub fn add_enhancement(mut self, enhancement: Enhancement) -> Self {
        self.enhancement = enhancement;
        self
    }

    pub fn add_edition(mut self, edition: Edition) -> Self {
        self.edition = edition;
        self
    }

    pub fn add_seal(mut self, seal: Seal) -> Self {
        self.seal = seal;
        self
    }

    pub fn on_trigger(&self, score: &mut Score) {
        let mut modifiers = Modifiers::chips(self.value);
        modifiers.add(&self.enhancement.modifiers());
        modifiers.add(&self.edition.modifiers());
        score.apply(&modifiers);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

#[derive(Clone, Copy)]
pub struct HandTypeProgression {
    pub chips_base: u32,
    pub chips_per_level: u32,
    pub mult_base: u32,
    pub mult_per_level: u32,
}

impl HandType {
    pub fn get_progression(&self) -> HandTypeProgression {
        match self {
            HandType::HighCard => HandTypeProgression {
                chips_base: 5,
                chips_per_level: 10,
                mult_base: 1,
                mult_per_level: 1,
            },
            HandType::Pair => HandTypeProgression {
                chips_base: 10,
                chips_per_level: 15,
                mult_base: 2,
                mult_per_level: 1,
            },
            HandType::TwoPair => HandTypeProgression {
                chips_base: 20,
                chips_per_level: 20,
                mult_base: 2,
                mult_per_level: 1,
            },
            HandType::ThreeOfAKind => HandTypeProgression {
                chips_base: 30,
                chips_per_level: 20,
                mult_base: 3,
                mult_per_level: 2,
            },
            HandType::Straight => HandTypeProgression {
                chips_base: 30,
                chips_per_level: 30,
                mult_base: 4,
                mult_per_level: 3,
            },
            HandType::Flush => HandTypeProgression {
                chips_base: 35,
                chips_per_level: 15,
                mult_base: 4,
                mult_per_level: 2,
            },
            HandType::FullHouse => HandTypeProgression {
                chips_base: 40,
                chips_per_level: 25,
                mult_base: 4,
                mult_per_level: 2,
            },
            HandType::FourOfAKind => HandTypeProgression {
                chips_base: 60,
                chips_per_level: 30,
                mult_base: 7,
                mult_per_level: 3,
            },
            HandType::StraightFlush => HandTypeProgression {
                chips_base: 100,
                chips_per_level: 40,
                mult_base: 8,
                mult_per_level: 4,
            },
            HandType::FiveOfAKind => HandTypeProgression {
                chips_base: 120,
                chips_per_level: 35,
                mult_base: 12,
                mult_per_level: 3,
            },
            HandType::FlushHouse => HandTypeProgression {
                chips_base: 140,
                chips_per_level: 40,
                mult_base: 14,
                mult_per_level: 4,
            },
            HandType::FlushFive => HandTypeProgression {
                chips_base: 160,
                chips_per_level: 50,
                mult_base: 16,
                mult_per_level: 3,
            },
        }
    }
}

pub struct BaseHand {
    hand_type: HandType,
    level: u32,
}

#[derive(Clone, Copy)]
pub struct BaseScore {
    pub chips: u32,
    pub mult: u32,
}

impl BaseHand {
    pub fn hand_type(&self) -> &HandType { &self.hand_type }
    pub fn level(&self) -> &u32 { &self.level }
}

impl BaseHand {
    pub fn new(hand_type: HandType, level: u32) -> Self {
        if level < 1 {
            panic!("Hand level must be greater than 0");
        }
        Self {
            hand_type,
            level,
        }
    }

    pub fn score(&self) -> BaseScore {
        let progression = self.hand_type.get_progression();
        BaseScore {
            chips: progression.chips_base + (self.level - 1) * progression.chips_per_level,
            mult: progression.mult_base + (self.level - 1) * progression.mult_per_level,
        }
    }
}
