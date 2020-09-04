use std::fmt;
use crate::field::Field;
use crate::card::Card;
use crate::player::Player;
use crate::position::{Offset, Position};
use crate::piece::Piece;
use crate::move_option::MoveOption;


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
		// move piece
		let piece: Option<Piece> = self.field.get_piece(&option.from_position).clone();
	    self.field.set_piece(&option.target_position, piece);
	    self.field.set_piece(&option.from_position, None);

	    // move cards around
	    let player_cards = &self.players[self.current_player].cards;
	    let card_index: usize = if player_cards[0].name == option.card.name {0} else {1};
	    self.players[self.current_player].cards[card_index] = self.public_card.clone();
	    self.public_card = option.card.clone();

	    // change active player
	    self.current_player = 1-self.current_player;
	}

	pub fn is_over(&self) -> bool {
		for (player_index, player) in self.players.iter().enumerate() {
			match self.field.get_master_position(player_index) {
				Some(position) => {
					if position.x == 2 && position.y == (1-player_index as isize) * 4 {
						println!("Game over. {}'s master has reached the opponent base", player.name);
						return true
					}
				},
				None => {
					println!("Game over. {}'s master has fallen", player.name);
					return true
				},
			};
		}
		false
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
