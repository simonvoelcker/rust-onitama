use std::fmt;
use crate::position::*;

#[derive(Clone)]
pub struct Card {
	pub name: &'static str,
	pub color: &'static str,
	pub moves: u32,
}

impl Card {
	pub const fn get_all_cards() -> [Card; 16] {
		return [
			Card {
				name: "Affe",
				color: "Blue",
				moves: MOVE_FL | MOVE_FR | MOVE_BL | MOVE_BR,
			},
			Card {
				name: "Drache",
				color: "Red",
				moves: MOVE_FLL | MOVE_FRR | MOVE_BL | MOVE_BR,
			},
			Card {
				name: "Elefant",
				color: "Red",
				moves: MOVE_FL | MOVE_FR | MOVE_L | MOVE_R,
			},
			Card {
				name: "Krabbe",
				color: "Blue",
				moves: MOVE_F | MOVE_LL | MOVE_RR,
			},
			Card {
				name: "Tiger",
				color: "Blue",
				moves: MOVE_FF | MOVE_B,
			},
			Card {
				name: "Gans",
				color: "Blue",
				moves: MOVE_FL | MOVE_L | MOVE_R | MOVE_BR,
			},
			Card {
				name: "Hahn",
				color: "Red",
				moves: MOVE_FR | MOVE_L | MOVE_R | MOVE_BL,
			},
			Card {
				name: "Ochse",
				color: "Blue",
				moves: MOVE_F | MOVE_R | MOVE_B,
			},
			Card {
				name: "Pferd",
				color: "Red",
				moves: MOVE_F | MOVE_L | MOVE_B,
			},
			Card {
				name: "Wildschwein",
				color: "Red",
				moves: MOVE_F | MOVE_L | MOVE_R,
			},
			Card {
				name: "Aal",
				color: "Blue",
				moves: MOVE_FL | MOVE_R | MOVE_BL,
			},
			Card {
				name: "Gottesanbeterin",
				color: "Red",
				moves: MOVE_FL | MOVE_FR | MOVE_B,
			},
			Card {
				name: "Kobra",
				color: "Red",
				moves: MOVE_FR | MOVE_L | MOVE_BR,
			},
			Card {
				name: "Kranich",
				color: "Blue",
				moves: MOVE_F | MOVE_BL | MOVE_BR,
			},
			Card {
				name: "Frosch",
				color: "Red",
				moves: MOVE_FL | MOVE_LL | MOVE_BR,
			},
			Card {
				name: "Hase",
				color: "Blue",
				moves: MOVE_FR | MOVE_RR | MOVE_BL,
			},
		];
	}
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}

impl std::cmp::PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
