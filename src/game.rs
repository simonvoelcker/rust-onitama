use std::fmt;
use crate::field::Field;
use crate::card::Card;
use crate::player::Player;
use crate::position::{Offset, Position};
use crate::piece::Piece;


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

	pub fn get_all_options(&self) -> Vec<(Position, usize, Position)> {
		let pieces: Vec<(Piece, Position)> = self.field.get_all_pieces(self.current_player);
		let cards = &self.players[self.current_player].cards;
		
		let mut options: Vec<(Position, usize, Position)> = Vec::new();
		for (_piece, position) in pieces.iter() {
			for (card_index, card) in cards.iter().enumerate() {
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
                    options.push((Position {x: position.x, y: position.y}, card_index, target_position));
				}
			}
		}
		options
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
