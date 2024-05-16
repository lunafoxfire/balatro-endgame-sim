use crate::sim::*;

#[derive(Clone, Copy)]
pub struct Modifiers {
    pub chips: u32,
    pub mult: u32,
    pub x_mult: f32,
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
