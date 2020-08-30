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
