#[cfg(test)]
#[path ="sim.test.rs"]
mod test;

#[derive(Clone, Copy)]
pub struct Modifiers {
    chips: u32,
    mult: u32,
    x_mult: f32,
}

impl Default for Modifiers {
    fn default() -> Self {
        Self {
            chips: 0,
            mult: 0,
            x_mult: 1.0,
        }
    }
}

impl Modifiers {
    pub fn new(chips: u32, mult: u32, x_mult: f32) -> Self {
        Self {
            chips,
            mult,
            x_mult,
        }
    }

    pub fn chips(val: u32) -> Self {
        Self {
            chips: val,
            ..Default::default()
        }
    }

    pub fn mult(val: u32) -> Self {
        Self {
            mult: val,
            ..Default::default()
        }
    }

    pub fn x_mult(val: f32) -> Self {
        Self {
            x_mult: val,
            ..Default::default()
        }
    }

    pub fn add(&mut self, modifiers: &Modifiers) {
        self.chips += modifiers.chips;
        self.mult += modifiers.mult;
        self.x_mult *= modifiers.x_mult;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Enhancement {
    None,
    Bonus,
    Mult,
    Glass,
}

impl Enhancement {
    pub fn modifiers(&self) -> Modifiers {
        match self {
            Self::None => { Modifiers::default() },
            Self::Bonus => { Modifiers::chips(30) },
            Self::Mult => { Modifiers::mult(4) },
            Self::Glass => { Modifiers::x_mult(2.0) },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Edition {
    None,
    Foil,
    Holo,
    Poly,
}

impl Edition {
    pub fn modifiers(&self) -> Modifiers {
        match self {
            Self::None => { Modifiers::default() },
            Self::Foil => { Modifiers::chips(50) },
            Self::Holo => { Modifiers::mult(10) },
            Self::Poly => { Modifiers::x_mult(1.5) },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Seal {
    None,
    Red,
}

pub enum JokerClass {
    Generic(Modifiers),
    GenericPerTrigger(Modifiers),
    Photograph,
    RetriggerAll, // Dusk, Sock and Buskin, Hack
    HangingChad,
    Mime,
    Baron,
}

pub struct Joker {
    class: JokerClass,
    edition: Edition,
}

impl Joker {
    pub fn class(&self) -> &JokerClass { &self.class }
    pub fn edition(&self) -> &Edition { &self.edition }
}

impl Joker {
    pub fn new(class: JokerClass) -> Self {
        Self {
            class,
            edition: Edition::None,
        }
    }

    pub fn add_edition(mut self, edition: Edition) -> Self {
        self.edition = edition;
        self
    }
}

impl Joker {
    pub fn on_setup(&self, retriggers: &mut Retriggers) {
        match self.class {
            JokerClass::RetriggerAll => {
                retriggers.all += 1;
            },
            JokerClass::HangingChad => {
                retriggers.first += 2;
            },
            JokerClass::Mime => {
                retriggers.hand += 1;
            },
            _ => {}
        }
    }

    pub fn on_card_trigger(&self, score: &mut Score, index: usize) {
        let modifiers = match self.class {
            JokerClass::GenericPerTrigger(modifiers) => { modifiers.clone() },
            JokerClass::Photograph => {
                if index == 0 { Modifiers::x_mult(2.0) }
                else {  Modifiers::default() }
            },
            _ => { Modifiers::default() }
        };
        score.apply(&modifiers);
    }

    pub fn on_unplayed_trigger(&self, score: &mut Score) {
        let modifiers = match self.class {
            JokerClass::Baron => { Modifiers::x_mult(1.5) },
            _ => { Modifiers::default() }
        };
        score.apply(&modifiers);
    }

    pub fn on_trigger(&self, score: &mut Score) {
        let mut modifiers = match self.class {
            JokerClass::Generic(modifiers) => { modifiers.clone() },
            _ => { Modifiers::default() }
        };
        modifiers.add(&self.edition.modifiers());
        score.apply(&modifiers);
    }
}

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
    chips_base: u32,
    chips_per_level: u32,
    mult_base: u32,
    mult_per_level: u32,
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
    chips: u32,
    mult: u32,
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

#[derive(Default, Clone, Copy)]
pub struct Score {
    chips: f64,
    mult: f64,
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
    all: u32,
    first: u32,
    hand: u32,
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
            if card.seal == Seal::Red {
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
