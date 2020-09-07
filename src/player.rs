use std::{fmt, cmp, hash};

use crate::card::Card;


#[derive(hash::Hash, cmp::PartialEq, cmp::Eq)]
pub struct Player {
	pub name: &'static str,
	pub cards: [&'static Card; 2],
}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}: {}, {}\n", self.name, self.cards[0], self.cards[1])
	}
}
