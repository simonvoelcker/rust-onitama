use std::fmt;

pub struct Position {
	pub x: isize,
	pub y: isize
}

impl Position {
	pub fn offset(&self, offset: &Offset) -> Position {
		Position {
			x: self.x + offset.x,
			y: self.y + offset.y
		}
	}

	pub fn in_field(&self) -> bool {
		self.x >= 0 && self.x < 5 && self.y >= 0 && self.y < 5
	}
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{}", self.x, self.y)
	}
}

pub struct Offset {
	pub x: isize,
	pub y: isize
}

impl fmt::Display for Offset {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{}", self.x, self.y)
	}
}
