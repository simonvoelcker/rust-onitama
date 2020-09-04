use std::fmt;
use crate::cell::Cell;
use crate::piece::Piece;
use crate::position::Position;


pub struct Field {
	pub cells: Vec<Vec<Cell>>
}

impl Field {
	pub fn new() -> Self {
		let mut field = Self {
			cells: vec![
				vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
			]
		};
		for col in 0..5 {
            field.cells[col][0] = Cell::Occupied(Piece {player: 0, is_master: col == 2});
            field.cells[col][4] = Cell::Occupied(Piece {player: 1, is_master: col == 2});
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

	pub fn get_cell(&self, position: &Position) -> &Cell {
		return &self.cells[position.x as usize][position.y as usize];
	}

	pub fn set_cell(mut self, position: &Position, cell: Cell) {
	    self.cells[position.x as usize][position.y as usize] = cell;
	}

	pub fn occupied_by(&self, position: &Position, player: usize) -> bool {
        let cell: &Cell = self.get_cell(&position);
        if let Cell::Occupied(piece) = cell {
            return piece.player == player
        }
        false
	}

	pub fn get_all_pieces(&self, player: usize) -> Vec<(Piece, Position)> {
		let mut pieces: Vec<(Piece, Position)> = Vec::new();
		for position in Field::get_all_positions() {
			let cell: &Cell = self.get_cell(&position);
			if let Cell::Occupied(piece) = cell {
				if piece.player == player {
					pieces.push((piece.clone(), position))
				}
			}
		}
		pieces
	}
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "    -----------\n")?;
		for y in 0..5 {
			write!(f, "{}  | ", 5-y)?;
			for x in 0..5 {
				let cell: &Cell = self.get_cell(&Position { x: x, y: 4-y });
				write!(f, "{} ", cell)?;
			}
			write!(f, "|\n")?;
		}
		write!(f, "    -----------\n")?;
		write!(f, "     A B C D E\n\n")
	}
}
