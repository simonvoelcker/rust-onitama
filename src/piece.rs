use std::fmt;

#[derive(Clone)]
pub struct Piece {
	pub player: usize,
	pub is_master: bool,
}

impl fmt::Display for Piece {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.player == 0 && self.is_master {
			write!(f, "M")
		} else if self.player == 0 && !self.is_master {
			write!(f, "A")
		} else if self.player == 1 && self.is_master {
			write!(f, "m")
		} else {
			write!(f, "a")
		}
	}
}
