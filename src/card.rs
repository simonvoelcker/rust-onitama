use std::fmt;
use crate::position::Offset;

pub struct Card {
	pub name: String,
	pub moves: Vec<Offset>
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}
