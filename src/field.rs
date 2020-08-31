use std::fmt;
use crate::cell::Cell;
use crate::piece::Piece;


pub struct Field {
	pub cells: Vec<Vec<Cell>>
}

impl Field {
	pub fn new() -> Self {
		Self {
			cells: vec![
				vec![
					Cell::Occupied(Piece {is_player: false, is_master: false}),
					Cell::Occupied(Piece {is_player: false, is_master: false}),
					Cell::Occupied(Piece {is_player: false, is_master: true}),
					Cell::Occupied(Piece {is_player: false, is_master: false}),
					Cell::Occupied(Piece {is_player: false, is_master: false})
				], 
				vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty], 
				vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty], 
				vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty], 
				vec![
					Cell::Occupied(Piece {is_player: true, is_master: false}),
					Cell::Occupied(Piece {is_player: true, is_master: false}),
					Cell::Occupied(Piece {is_player: true, is_master: true}),
					Cell::Occupied(Piece {is_player: true, is_master: false}),
					Cell::Occupied(Piece {is_player: true, is_master: false})
				],
			]
		}
	}

	pub fn get_all_pieces(self, is_player: bool) -> Vec<(Piece, usize, usize)> {
		let mut pieces: Vec<(Piece, usize, usize)> = Vec::new();
		for (row_idx, row) in self.cells.iter().enumerate() {
			for (col_idx, cell) in row.iter().enumerate() {
				if let Cell::Occupied(piece) = cell {
					if piece.is_player == is_player {
						pieces.push((piece.clone(), col_idx, row_idx))
					}
				}
			}
		}
		pieces
	}
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for row in self.cells.iter() {
			for cell in row.iter() {
				write!(f, "{} ", cell).expect("");
			}
			write!(f, "\n").expect("");
		}
		write!(f, "\n")
	}
}
