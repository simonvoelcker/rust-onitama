use std::fmt;
use crate::cell::Cell;
use crate::piece::Piece;
use crate::position::Position;
use crate::card::Card;


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

	pub fn get_all_positions() -> Vec<Position> {
		let mut positions: Vec<Position> = Vec::new();
		for x in 0..5 {
			for y in 0..5 {
				positions.push(Position { x, y })
			}
		}
		positions
	}

	pub fn get_cell(&self, position: &Position) -> &Cell {
		return &self.cells[position.x as usize][position.y as usize];
	}

	pub fn get_all_pieces(&self, is_player: bool) -> Vec<(Piece, Position)> {
		let mut pieces: Vec<(Piece, Position)> = Vec::new();
		for position in Field::get_all_positions() {
			let cell: &Cell = self.get_cell(&position);
			if let Cell::Occupied(piece) = cell {
				if piece.is_player == is_player {
					pieces.push((piece.clone(), position))
				}
			}
		}
		pieces
	}

	pub fn get_all_moves(&self, piece: &Piece, position: &Position, card: &Card) -> Vec<Position> {
		let mut targets: Vec<Position> = Vec::new();
		for offset in card.moves.iter() {
			let target = position.offset(&offset);
			if target.in_field() {
				targets.push(target)
			}
		}
		targets
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
