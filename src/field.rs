use std::fmt;
use crate::piece::Piece;
use crate::position::Position;


pub struct Field {
	pub pieces: [Option<&'static Piece>; 25],
}

impl Field {
	pub fn new(pieces: [&'static Piece; 10]) -> Self {
		let mut field = Self {pieces: Default::default()};
		for col in 0..5 {
            field.set_piece(&Position {x: col, y: 0}, Some(pieces[col as usize]));
            field.set_piece(&Position {x: col, y: 4}, Some(pieces[col as usize + 5]));
		}
		field
	}

	pub fn get_piece(&self, position: &Position) -> Option<&'static Piece> {
		return self.pieces[position.field_index()];
	}

	pub fn set_piece(&mut self, position: &Position, piece: Option<&'static Piece>) {
	    self.pieces[position.field_index()] = piece;
	}

	pub fn occupied_by(&self, position: &Position, player: usize) -> bool {
        if let Some(piece) = self.get_piece(&position) {
        	return piece.player == player;
        }
        false
	}

	pub fn get_all_pieces(&self, player: usize) -> Vec<(&'static Piece, Position)> {
		let mut pieces: Vec<(&'static Piece, Position)> = Vec::new();
		for field_index in 0..25 {
	        if let Some(piece) = &self.pieces[field_index] {
				if piece.player == player {
					pieces.push((piece, Position::from_field_index(field_index)));
				}
	        }
		}
		pieces
	}

	pub fn get_master_position(&self, player: usize) -> Option<Position> {
		for field_index in 0..25 {
	        if let Some(piece) = &self.pieces[field_index] {
				if piece.player == player && piece.is_master {
					return Some(Position::from_field_index(field_index));
				}
	        }
		}
		None
	}
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "    -----------\n")?;
		for y in 0..5 {
			write!(f, "{}  | ", 5-y)?;
			for x in 0..5 {
				let piece: Option<&'static Piece> = self.get_piece(&Position { x: x, y: 4-y });
				match piece {
					Some(piece) => {write!(f, "{} ", piece)?},
					None => {write!(f, "- ")?},
				};
			}
			write!(f, "|\n")?;
		}
		write!(f, "    -----------\n")?;
		write!(f, "     A B C D E\n\n")
	}
}
