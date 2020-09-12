use std::{fmt, cmp};
use serde::{Serialize};

use crate::piece::Piece;
use crate::position::Position;


#[derive(cmp::PartialEq, cmp::Eq, Serialize)]
pub struct Field {
	pieces: [Option<&'static Piece>; 25],
}

static PIECES: [Piece; 4] = [
	Piece {player: 0, is_master: false},
	Piece {player: 0, is_master: true},
	Piece {player: 1, is_master: false},
	Piece {player: 1, is_master: true},
];

fn get_piece(player: u64, is_master: bool) -> &'static Piece {
	let index = (player << 1) | (is_master as u64);
	return &PIECES[index as usize];
}

impl Field {
	pub fn new() -> Self {
		let mut field = Self {pieces: Default::default()};
		for col in 0..5 {
            field.set_piece(&Position {x: col, y: 0}, Some(get_piece(0, col == 2)));
            field.set_piece(&Position {x: col, y: 4}, Some(get_piece(1, col == 2)));
		}
		field
	}

	pub fn pack(&self) -> u64 {
		// produce a packed representation of the field.
		// the result fits into 10*3 + 15*1 = 45 bits.
		let mut packed: u64 = 0;
		for field_index in 0..25 {
			match self.pieces[field_index] {
				Some(piece) => {
					packed = (packed << 1) | (piece.player as u64);
					packed = (packed << 1) | (piece.is_master as u64);
					packed = (packed << 1) | 1;
				},
				None => {
					packed <<= 1;
				},
			};
		}
		packed
	}

	// pub fn unpack(packed_game: u64) -> Self {
	// 	let mut packed = packed_game;
	// 	// counterpart to ::pack
	// 	let mut field = Self {pieces: Default::default()};
	// 	for _field_index in 0..25 {
	// 		if (packed & 1) > 0 {
	// 			packed >>= 1;
	// 			let _is_master = (packed & 1) > 0;
	// 			packed >>= 1;
	// 			let _player = (packed & 1) as u64;
	// 			packed >>= 1;	
	// 		} else {
	// 			packed >>= 1;
	// 		}
	// 	}
	// 	field
	// }

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

	pub fn get_piece_balance(&self, player: usize) -> isize {
		let mut balance: isize = 0;
		for field_index in 0..25 {
	        if let Some(piece) = &self.pieces[field_index] {
				balance += if piece.player == player {1} else {-1};
	        }
		}
		balance
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
