use std::fmt;
use crate::field::Field;
use crate::card::Card;
use crate::player::Player;
use crate::position::Position;
use crate::piece::Piece;
use crate::move_option::MoveOption;


pub struct Game {
	pub field: Field,
	pub players: [Player; 2],
	pub public_card: &'static Card,
	pub current_player: usize,
}

pub enum GameResult {
	DecidedWithWinner(usize),
	Undecided,
}

impl Game {
	pub fn new(players: [Player; 2], public_card: &'static Card) -> Self {
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
				for offset in card.get_offsets(self.current_player == 1).iter() {
					let target_position = position.offset(offset);
					if !target_position.in_field() {
					    continue;
					}
					if self.field.occupied_by(&target_position, self.current_player) {
					    continue;
					}
                    options.push(MoveOption {
                    	from_position: position.clone(),
                    	card: card,
                    	target_position: target_position.clone(),
                    	target_piece: self.field.get_piece(&target_position).clone(),
                    	public_card: self.public_card,
                    });
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

	    // move cards
	    if self.players[self.current_player].cards[0] == option.card {
		    self.players[self.current_player].cards[0] = self.public_card;
	    } else {
		    self.players[self.current_player].cards[1] = self.public_card;
	    }
	    self.public_card = option.card;

	    // change active player
	    self.current_player = 1-self.current_player;
	}

	pub fn undo_move(&mut self, option: &MoveOption) {
	    // change active player
	    self.current_player = 1-self.current_player;

	    // move cards back
	    if self.players[self.current_player].cards[0] == option.public_card {
		    self.public_card = self.players[self.current_player].cards[0];
		    self.players[self.current_player].cards[0] = option.card;
	    } else {
		    self.public_card = self.players[self.current_player].cards[1];
		    self.players[self.current_player].cards[1] = option.card;
	    }

		// move pieces back
		let piece: Option<Piece> = self.field.get_piece(&option.target_position).clone();
	    self.field.set_piece(&option.from_position, piece);
	    self.field.set_piece(&option.target_position, option.target_piece.clone());
	}

	pub fn get_result(&self) -> GameResult {
		for (player_index, _player) in self.players.iter().enumerate() {
			match self.field.get_master_position(player_index) {
				Some(position) => {
					if position.x == 2 && position.y == (1-player_index as isize) * 4 {
						return GameResult::DecidedWithWinner(player_index);
					}
				},
				None => {
					return GameResult::DecidedWithWinner(1-player_index);
				},
			};
		}
		GameResult::Undecided
	}

	pub fn evaluate_move(&mut self, option: &MoveOption, depth: usize) -> f64 {
		self.make_move(option);
		let mut score = 0.5;

		match self.get_result() {
			GameResult::DecidedWithWinner(winning_player) => {
				// move was made -> current player is OPPONENT
				score = if self.current_player == winning_player {0.0} else {1.0}
			},
			GameResult::Undecided => {
				if depth > 0 {
					let mut max_score = 0.0;
					let mut tot_score = 0.0;
					let all_options = self.get_all_options();
				    for option in all_options.iter() {
				    	let option_score = self.evaluate_move(&option, depth-1);
						if option_score > max_score {
							max_score = option_score;
						}
						tot_score += option_score;
				    }
				    if max_score == 1.0 {
				    	score = 0.0;
				    } else {
				    	score = 1.0 - tot_score / all_options.len() as f64;
				    }
				}
			}
		};

		self.undo_move(option);
		score
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
