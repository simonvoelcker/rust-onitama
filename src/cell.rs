use std::fmt;
use crate::piece::Piece;


pub enum Cell {
	Empty,
	Occupied(Piece)
}

impl fmt::Display for Cell {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Cell::Empty => write!(f, "-"),
			Cell::Occupied(piece) => write!(f, "{}", piece)
		}
	}
}
