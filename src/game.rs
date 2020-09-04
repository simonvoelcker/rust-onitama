use std::fmt;
use crate::field::Field;
use crate::card::Card;
use crate::player::Player;
use crate::position::{Offset, Position};
use crate::piece::Piece;
use crate::move_option::MoveOption;
use crate::cell::Cell;


pub struct Game {
	pub field: Field,
	pub players: [Player; 2],
	pub public_card: Card,
	pub current_player: usize,
}

impl Game {
	pub fn new(players: [Player; 2], public_card: Card) -> Self {
		let player_0_starts = players[0].name.starts_with(&public_card.color);
		Self {
			field: Field::new(),
			players,
			public_card,
			current_player: if player_0_starts {0} else {1},
		}
	}

	pub fn get_all_options(&self) -> Vec<MoveOption> {
		let pieces: Vec<(Piece, Position)> = self.field.get_all_pieces(self.current_player);
		let cards = &self.players[self.current_player].cards;
		
		let mut options: Vec<MoveOption> = Vec::new();
		for (_piece, position) in pieces.iter() {
			for card in cards.iter() {
				for offset in card.moves.iter() {
				    let player_offset = if self.current_player == 0 {
                        Offset {x: offset.x, y: offset.y}
				    } else {
                        Offset {x: -offset.x, y: -offset.y}
				    };
					let target_position = position.offset(&player_offset);
					if !target_position.in_field() {
					    continue;
					}
					if self.field.occupied_by(&target_position, self.current_player) {
					    continue;
					}
                    options.push(MoveOption {from_position: position.clone(), card: card.clone(), target_position});
				}
			}
		}
		options
	}

	pub fn make_move(&mut self, option: &MoveOption) {
	    let position = &option.from_position;
	    self.field.set_cell(position, Cell::Empty);
	}
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\n{}", self.players[1])?;
		if self.current_player == 1 {
			write!(f, "Public: {}\n", self.public_card)?;
		}
		write!(f, "\n{}", self.field)?;
		if self.current_player == 0 {
			write!(f, "Public: {}\n", self.public_card)?;
		}
		write!(f, "{}\n", self.players[0])
	}
}
