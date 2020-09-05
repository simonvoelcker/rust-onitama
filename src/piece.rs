use std::fmt;

pub struct Piece {
	pub player: usize,
	pub is_master: bool,
}

impl Piece {
	pub const fn get_all_pieces() -> [Piece; 10] {
		return [
			Piece {player: 0, is_master: false},
			Piece {player: 0, is_master: false},
			Piece {player: 0, is_master: true},
			Piece {player: 0, is_master: false},
			Piece {player: 0, is_master: false},
			Piece {player: 1, is_master: false},
			Piece {player: 1, is_master: false},
			Piece {player: 1, is_master: true},
			Piece {player: 1, is_master: false},
			Piece {player: 1, is_master: false},
		]
	}
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
