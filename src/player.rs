use std::{fmt, cmp};

use crate::card::Card;


#[derive(cmp::PartialEq, cmp::Eq)]
pub struct Player {
	pub color: &'static str,
	pub cards: [&'static Card; 2],
	pub bot_strength: Option<u64>,
}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} player: {}, {}\n", self.color, self.cards[0], self.cards[1])
	}
}
