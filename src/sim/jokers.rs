use crate::sim::*;

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
