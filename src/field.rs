use std::fmt;
use crate::piece::Piece;
use crate::position::Position;


pub struct Field {
	pub pieces: Vec<Vec<Option<Piece>>>
}

impl Field {
	pub fn new() -> Self {
		let mut field = Self {
			pieces: vec![
				vec![None, None, None, None, None],
				vec![None, None, None, None, None],
				vec![None, None, None, None, None],
				vec![None, None, None, None, None],
				vec![None, None, None, None, None],
			]
		};
		for col in 0..5 {
            field.pieces[col][0] = Some(Piece {player: 0, is_master: col == 2});
            field.pieces[col][4] = Some(Piece {player: 1, is_master: col == 2});
		}
		field
	}

	pub fn get_all_positions() -> Vec<Position> {
		let mut positions: Vec<Position> = Vec::new();
		for x in 0..5 {
			for y in 0..5 {
				positions.push(Position { x, y })
			}
		}
		positions
	}

	pub fn get_piece(&self, position: &Position) -> &Option<Piece> {
		return &self.pieces[position.x as usize][position.y as usize];
	}

	pub fn set_piece(&mut self, position: &Position, piece: Option<Piece>) {
	    self.pieces[position.x as usize][position.y as usize] = piece;
	}

	pub fn occupied_by(&self, position: &Position, player: usize) -> bool {
        if let Some(piece) = self.get_piece(&position) {
        	return piece.player == player;
        }
        false
	}

	pub fn get_all_pieces(&self, player: usize) -> Vec<(Piece, Position)> {
		let mut pieces: Vec<(Piece, Position)> = Vec::new();
		for position in Field::get_all_positions() {
	        match self.get_piece(&position) {
	        	Some(piece) => {
					if piece.player == player {
						pieces.push((piece.clone(), position))
					}
	        	},
        		None => {()},
	        };
		}
		pieces
	}

	pub fn get_master_position(&self, player: usize) -> Option<Position> {
		for position in Field::get_all_positions() {
	        match self.get_piece(&position) {
	        	Some(piece) => {
					if piece.player == player && piece.is_master {
						return Some(position);
					}
	        	},
        		None => {()},
	        };
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
				let piece: &Option<Piece> = self.get_piece(&Position { x: x, y: 4-y });
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
