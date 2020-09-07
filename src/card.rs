use std::{fmt, cmp, hash};

use crate::position::Offset;

#[derive(hash::Hash)]
pub struct Card {
	pub name: &'static str,
	pub color: &'static str,
	pub moves: u32,
}

// Front, Back, Left, Right moves.
// The corresponding bit is read from an ideal 5x5 grid, rowwise,
// but only existing moves are considered here, so some are missing.
const MOVE_FF:   u32 = 1 << 2;

const MOVE_FLL:  u32 = 1 << 5;
const MOVE_FL:   u32 = 1 << 6;
const MOVE_F:    u32 = 1 << 7;
const MOVE_FR:   u32 = 1 << 8;
const MOVE_FRR:  u32 = 1 << 9;

const MOVE_LL:   u32 = 1 << 10;
const MOVE_L:    u32 = 1 << 11;

const MOVE_R:    u32 = 1 << 13;
const MOVE_RR:   u32 = 1 << 14;

const MOVE_BL:   u32 = 1 << 16;
const MOVE_B:    u32 = 1 << 17;
const MOVE_BR:   u32 = 1 << 18;


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

	pub fn get_offsets(&self, invert: bool) -> Vec<Offset> {
		let mut offsets: Vec<Offset> = Vec::new();
		for bit_index in 0..25 {
			if self.moves & (1 << bit_index) > 0 {
				let x = ((bit_index % 5) - 2) * (if invert {-1} else {1});
				let y = (2 - (bit_index / 5)) * (if invert {-1} else {1});
				offsets.push(Offset {x, y});
			}
		}
		offsets
	}
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}

impl cmp::PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl cmp::Eq for Card {}
