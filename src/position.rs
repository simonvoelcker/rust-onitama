use std::fmt;

#[derive(Clone)]
pub struct Position {
	pub x: isize,
	pub y: isize
}

impl Position {
	pub fn offset(&self, offset: &Offset) -> Position {
		Position {
			x: self.x + offset.x,
			y: self.y + offset.y,
		}
	}

	pub fn in_field(&self) -> bool {
		self.x >= 0 && self.x < 5 && self.y >= 0 && self.y < 5
	}

	pub fn field_index(&self) -> usize {
		(self.y * 5 + self.x) as usize
	}

	pub fn from_field_index(field_index: usize) -> Position {
		Position {
			x: (field_index % 5) as isize,
			y: (field_index / 5) as isize,
		}
	}
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", ('A' as u8 + self.x as u8) as char, self.y+1)
	}
}

#[derive(Clone)]
pub struct Offset {
	pub x: isize,
	pub y: isize
}

impl fmt::Display for Offset {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{}", self.x, self.y)
	}
}
