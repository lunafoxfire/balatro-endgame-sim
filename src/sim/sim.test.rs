use super::*;

#[test]
fn base_hand_score() {
    {
        let base_hand = BaseHand::new(HandType::HighCard, 1);
        let hand = vec![
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 15.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 28380.0);
    }
}

#[test]
fn bonus_cards() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Bonus),
            Card::new(10).add_enhancement(Enhancement::Bonus),
            Card::new(10).add_enhancement(Enhancement::Bonus),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 32250.0);
    }
}

#[test]
fn mult_cards() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Mult),
            Card::new(10).add_enhancement(Enhancement::Mult),
            Card::new(10).add_enhancement(Enhancement::Mult),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 36300.0);
    }
}

#[test]
fn glass_cards() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Glass),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 56760.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Glass),
            Card::new(10).add_enhancement(Enhancement::Glass),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 113520.0);
    }
}

#[test]
fn foil_cards() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_edition(Edition::Foil),
            Card::new(10).add_edition(Edition::Foil),
            Card::new(10).add_edition(Edition::Foil),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 34830.0);
    }
}

#[test]
fn holographic_cards() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_edition(Edition::Holo),
            Card::new(10).add_edition(Edition::Holo),
            Card::new(10).add_edition(Edition::Holo),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 48180.0);
    }
}

#[test]
fn polychrome_cards() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_edition(Edition::Poly),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 42570.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_edition(Edition::Poly),
            Card::new(10).add_edition(Edition::Poly),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 63855.0);
    }
}

#[test]
fn hand_order_of_operations() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Mult),
            Card::new(10).add_enhancement(Enhancement::Glass),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 62040.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Glass),
            Card::new(10).add_enhancement(Enhancement::Mult),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 59400.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_edition(Edition::Holo),
            Card::new(10).add_edition(Edition::Poly),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 52470.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_edition(Edition::Poly),
            Card::new(10).add_edition(Edition::Holo),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 49170.0);
    }
}

#[test]
fn card_order_of_operations() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Glass).add_edition(Edition::Holo),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 69960.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Mult).add_edition(Edition::Poly),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 46530.0);
    }
}

#[test]
fn red_seal() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_seal(Seal::Red),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 28810.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_edition(Edition::Holo).add_seal(Seal::Red),
            Card::new(10).add_enhancement(Enhancement::Mult).add_seal(Seal::Red),
            Card::new(10).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
            Card::new(10).add_enhancement(Enhancement::Glass).add_edition(Edition::Poly).add_seal(Seal::Red),
            Card::new(10).add_enhancement(Enhancement::Glass).add_seal(Seal::Red),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 7259040.0);
    }
}

#[test]
fn steel_cards() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![
            UnplayedCard::steel(),
        ];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 42570.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![];
        let unplayed_cards = vec![
            UnplayedCard::steel(),
            UnplayedCard::steel_king(),
            UnplayedCard::king(),
            UnplayedCard::steel().add_seal(),
        ];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 143673.75);
    }
}

#[test]
fn generic_joker() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Generic(Modifiers::chips(50)))
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 30530.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Generic(Modifiers::mult(50)))
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 61380.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Generic(Modifiers::x_mult(4.0)))
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 113520.0);
    }
}

#[test]
fn foil_joker() {
        {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Generic(Modifiers::mult(50))).add_edition(Edition::Foil)
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 66030.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Generic(Modifiers::x_mult(4.0))).add_edition(Edition::Foil)
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 122120.0);
    }
}

#[test]
fn holographic_joker() {
        {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Generic(Modifiers::mult(50))).add_edition(Edition::Holo)
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 67980.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Generic(Modifiers::x_mult(4.0))).add_edition(Edition::Holo)
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 139920.0);
    }
}

#[test]
fn polychrome_joker() {
        {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Generic(Modifiers::mult(50))).add_edition(Edition::Poly)
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 92070.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Generic(Modifiers::x_mult(4.0))).add_edition(Edition::Poly)
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 170280.0);
    }
}

#[test]
fn generic_per_trigger_joker() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::GenericPerTrigger(Modifiers::chips(50)))
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 39130.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::GenericPerTrigger(Modifiers::mult(50)))
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 193380.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::GenericPerTrigger(Modifiers::x_mult(1.5)))
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 215510.625);
    }
}

#[test]
fn retrigger_joker() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Mult),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::RetriggerAll)
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 36210.0);
    }
}

#[test]
fn hanging_chad_joker() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Mult),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::HangingChad)
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 37400.0);
    }
}

#[test]
fn photograph_joker() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Mult),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Photograph)
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 62040.0);
    }
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10).add_enhancement(Enhancement::Mult),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Photograph),
            Joker::new(JokerClass::HangingChad),
        ];
        let unplayed_cards = vec![];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 272000.0);
    }
}

#[test]
fn baron_joker() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Baron)
        ];
        let unplayed_cards = vec![
            UnplayedCard::king(),
            UnplayedCard::king().add_seal(),
            UnplayedCard::steel(),
        ];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 143673.75);
    } 
}

#[test]
fn mime_joker() {
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Mime)
        ];
        let unplayed_cards = vec![
            UnplayedCard::steel(),
            UnplayedCard::steel(),
            UnplayedCard::steel(),
        ];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 323265.9375);
    } 
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Baron),
            Joker::new(JokerClass::Mime),
        ];
        let unplayed_cards = vec![
            UnplayedCard::steel_king(),
            UnplayedCard::steel_king(),
            UnplayedCard::steel_king(),
        ];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 3682201.0693359375);
    } 
    {
        let base_hand = BaseHand::new(HandType::FlushFive, 10);
        let hand = vec![
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
            Card::new(10),
        ];
        let jokers = vec![
            Joker::new(JokerClass::Baron),
            Joker::new(JokerClass::Mime),
        ];
        let unplayed_cards = vec![
            UnplayedCard::steel_king(),
            UnplayedCard::steel_king().add_seal(),
        ];
        let mut sim = Simulation::init(base_hand, hand, jokers, unplayed_cards);
        assert_eq!(sim.run(), 1636533.80859375);
    } 
}
