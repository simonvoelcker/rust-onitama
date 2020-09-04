use std::fmt;

use crate::position::Position;
use crate::card::Card;
use crate::piece::Piece;

pub struct MoveOption {
	pub from_position: Position,
	pub card: Card,
	pub target_position: Position,
	pub target_piece: Option<Piece>,
}

impl fmt::Display for MoveOption {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let kill = match &self.target_piece {
			Some(piece) => { if piece.is_master {"X"} else {"x"} },
			None => {""},
		};
		write!(f, "{} -> {} (using {}) {}", self.from_position, self.target_position, self.card, kill)
	}
}
