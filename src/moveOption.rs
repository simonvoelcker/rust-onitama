use std::fmt;

use crate::position::Position;

pub struct MoveOption {
	pub from_position: Position,
	pub card_index: usize,
	pub target_position: Position,
}

impl fmt::Display for MoveOption {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} -> {} (using {})", self.from_position, self.target_position, self.card_index)
	}
}
