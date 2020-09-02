use std::fmt;

use crate::card::Card;

pub struct Player {
	pub name: String,
	pub cards: (Card, Card),
}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}: {}, {}\n", self.name, self.cards.0, self.cards.1)
	}
}
