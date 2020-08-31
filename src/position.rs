use std::fmt;

pub struct Position {
	pub x: usize,
	pub y: usize
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{}", self.x, self.y)
	}
}
