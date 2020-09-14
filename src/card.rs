use std::{fmt, cmp};
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

use crate::position::Offset;

#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
	pub name: String,
	pub color: String,
	pub moves: u32,
}

impl Card {
	pub fn get_offsets(&self, invert: bool) -> Vec<Offset> {
		// find the MOVE_ constants below for an explanation of what this does
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

impl cmp::Ord for Card {
	fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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


pub fn get_card_by_index(index: usize) -> Card {
	[
		Card {
			name: "Affe".to_string(),
			color: "Blue".to_string(),
			moves: MOVE_FL | MOVE_FR | MOVE_BL | MOVE_BR,
		},
		Card {
			name: "Drache".to_string(),
			color: "Red".to_string(),
			moves: MOVE_FLL | MOVE_FRR | MOVE_BL | MOVE_BR,
		},
		Card {
			name: "Elefant".to_string(),
			color: "Red".to_string(),
			moves: MOVE_FL | MOVE_FR | MOVE_L | MOVE_R,
		},
		Card {
			name: "Krabbe".to_string(),
			color: "Blue".to_string(),
			moves: MOVE_F | MOVE_LL | MOVE_RR,
		},
		Card {
			name: "Tiger".to_string(),
			color: "Blue".to_string(),
			moves: MOVE_FF | MOVE_B,
		},
		Card {
			name: "Gans".to_string(),
			color: "Blue".to_string(),
			moves: MOVE_FL | MOVE_L | MOVE_R | MOVE_BR,
		},
		Card {
			name: "Hahn".to_string(),
			color: "Red".to_string(),
			moves: MOVE_FR | MOVE_L | MOVE_R | MOVE_BL,
		},
		Card {
			name: "Ochse".to_string(),
			color: "Blue".to_string(),
			moves: MOVE_F | MOVE_R | MOVE_B,
		},
		Card {
			name: "Pferd".to_string(),
			color: "Red".to_string(),
			moves: MOVE_F | MOVE_L | MOVE_B,
		},
		Card {
			name: "Wildschwein".to_string(),
			color: "Red".to_string(),
			moves: MOVE_F | MOVE_L | MOVE_R,
		},
		Card {
			name: "Aal".to_string(),
			color: "Blue".to_string(),
			moves: MOVE_FL | MOVE_R | MOVE_BL,
		},
		Card {
			name: "Gottesanbeterin".to_string(),
			color: "Red".to_string(),
			moves: MOVE_FL | MOVE_FR | MOVE_B,
		},
		Card {
			name: "Kobra".to_string(),
			color: "Red".to_string(),
			moves: MOVE_FR | MOVE_L | MOVE_BR,
		},
		Card {
			name: "Kranich".to_string(),
			color: "Blue".to_string(),
			moves: MOVE_F | MOVE_BL | MOVE_BR,
		},
		Card {
			name: "Frosch".to_string(),
			color: "Red".to_string(),
			moves: MOVE_FL | MOVE_LL | MOVE_BR,
		},
		Card {
			name: "Hase".to_string(),
			color: "Blue".to_string(),
			moves: MOVE_FR | MOVE_RR | MOVE_BL,
		},
	][index].clone()
}
