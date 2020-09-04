use std::fmt;

use crate::position::Position;
use crate::card::Card;

pub struct MoveOption {
	pub from_position: Position,
	pub card: Card,
	pub target_position: Position,
}

impl fmt::Display for MoveOption {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} -> {} (using {})", self.from_position, self.target_position, self.card)
	}
}
