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

impl Offset {
	pub fn from_moves(moves: u32) -> Vec<Offset> {
		let mut offsets: Vec<Offset> = Vec::new();
		for bit_index in 0..25 {
			if moves & (1 << bit_index) > 0 {
				let x = (bit_index % 5) - 2;
				let y = 2 - (bit_index / 5);
				offsets.push(Offset {x, y});
			}
		}
		offsets
	}
}

impl fmt::Display for Offset {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{}", self.x, self.y)
	}
}

// Front, Back, Left, Right moves.
// The corresponding bit is read from an ideal 5x5 grid, rowwise,
// but only existing moves are considered here, so some are missing.
pub const MOVE_FF:   u32 = 1 << 2;

pub const MOVE_FLL:  u32 = 1 << 5;
pub const MOVE_FL:   u32 = 1 << 6;
pub const MOVE_F:    u32 = 1 << 7;
pub const MOVE_FR:   u32 = 1 << 8;
pub const MOVE_FRR:  u32 = 1 << 9;

pub const MOVE_LL:   u32 = 1 << 10;
pub const MOVE_L:    u32 = 1 << 11;

pub const MOVE_R:    u32 = 1 << 13;
pub const MOVE_RR:   u32 = 1 << 14;

pub const MOVE_BL:   u32 = 1 << 16;
pub const MOVE_B:    u32 = 1 << 17;
pub const MOVE_BR:   u32 = 1 << 18;
