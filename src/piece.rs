use std::fmt;

pub struct Piece {
	pub is_player: bool,
	pub is_master: bool
}

impl fmt::Display for Piece {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.is_player && self.is_master {
			write!(f, "M")
		} else if self.is_player && !self.is_master {
			write!(f, "A")
		} else if !self.is_player && self.is_master {
			write!(f, "m")
		} else {
			write!(f, "a")
		}
	}
}
