use std::io;
use std::io::Write;
use std::{fmt, cmp, hash};
use std::collections::HashMap;

use crate::field::Field;
use crate::card::Card;
use crate::player::Player;
use crate::position::Position;
use crate::piece::Piece;
use crate::move_option::MoveOption;


#[derive(hash::Hash, cmp::PartialEq, cmp::Eq)]
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

static PIECES: [Piece; 10] = Piece::get_all_pieces();

impl Game {
	pub fn new(players: [Player; 2], public_card: &'static Card) -> Self {
		let player_0_starts = players[0].name.starts_with(&public_card.color);
		let pieces = [
			&PIECES[0], &PIECES[1], &PIECES[2], &PIECES[3], &PIECES[4],
			&PIECES[5], &PIECES[6], &PIECES[7], &PIECES[8], &PIECES[9],
		];
		Self {
			field: Field::new(pieces),
			players,
			public_card,
			current_player: if player_0_starts {0} else {1},
		}
	}

	pub fn compress(&self) -> u64 {
		// produce a compressed representation of the game. result fits into 64 bits.
		// current player is 1 bit, the cards 10 bits, the field 45 bits.
		let mut compressed: u64 = self.current_player as u64;
		compressed = (compressed << 10) | self.compress_cards();
		compressed = (compressed << 45) | self.field.compress();
		compressed
	}

	fn compress_cards(&self) -> u64 {
		let mut compressed: u64 = 0;
		// cards selection fits into 10 bits: given the sorted list of all cards in the game,
		// each card is either public, or player 1's, or player 2's (=2 bit each).
		let mut cards: [&'static Card; 5] = [
			self.public_card,
			self.players[0].cards[0],
			self.players[0].cards[1],
			self.players[1].cards[0],
			self.players[1].cards[1],
		];
		cards.sort();  // TODO sorting references may not act as I hope it will
		for card in cards.iter() {
			if card == &self.public_card {
				compressed = compressed << 2;
			} else if card == &self.players[0].cards[0] || card == &self.players[0].cards[1] {
				compressed = (compressed << 2) | 0x02;
			} else {
				compressed = (compressed << 2) | 0x03;
			}
		}
		compressed
	}

	pub fn get_all_options(&self) -> Vec<MoveOption> {
		let pieces: Vec<(&'static Piece, Position)> = self.field.get_all_pieces(self.current_player);
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
                    	target_piece: self.field.get_piece(&target_position),
                    	public_card: self.public_card,
                    });
				}
			}
		}
		options
	}

	pub fn make_move(&mut self, option: &MoveOption) {
		// move piece
		let piece: Option<&'static Piece> = self.field.get_piece(&option.from_position);
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
		let piece: Option<&'static Piece> = self.field.get_piece(&option.target_position);
	    self.field.set_piece(&option.from_position, piece);
	    self.field.set_piece(&option.target_position, option.target_piece);
	}

	pub fn run_turn(&mut self) {
		if self.players[self.current_player].is_bot {
			self.make_bot_move();
		} else {
			self.make_human_move();
		}
	}

	fn make_bot_move(&mut self) {
	    let options: Vec<MoveOption> = self.get_all_options();
	    let depth: usize = self.propose_evaluation_depth(5000000);
		print!("Bot has {} options. Evaluating (depth {}) .", options.len(), depth);

		let mut score_cache: HashMap<u64, f64> = HashMap::new();
		let mut highest_score: f64 = 0.0;
		let mut best_option: &MoveOption = &options[0];
		io::stdout().flush().unwrap();
		for option in options.iter() {
			let score = self.evaluate_move(&option, depth, &mut score_cache);
			if score > highest_score {
				highest_score = score;
				best_option = &option;
			}
			print!(".");
			io::stdout().flush().unwrap();
		}
		println!("\nBot's move: {} (Score is {:.1})", *best_option, highest_score);
		self.make_move(&best_option);
	}

	fn make_human_move(&mut self) {
	    let options: Vec<MoveOption> = self.get_all_options();
		println!("You have {} options:", options.len());
	    for (option_index, option) in options.iter().enumerate() {
		    println!("Option {:2}: {}", option_index+1, option);
	    }

	    let mut choice = 0;
	    while choice < 1 || choice > options.len() {
		    print!("Choose option: ");
		    io::stdout().flush().unwrap();
		    let mut input = String::new();
		    io::stdin().read_line(&mut input).unwrap();
		    match input.trim().parse() {
		    	Ok(num) => {choice = num},
		    	Err(_) => {choice = 0},
		    };
	    }
	    self.make_move(&options[choice-1]);
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

	fn evaluate_move(&mut self, option: &MoveOption, depth: usize, score_cache: &mut HashMap<u64, f64>) -> f64 {
		self.make_move(option);
		let score;

		match self.get_result() {
			GameResult::DecidedWithWinner(winning_player) => {
				// move was made -> current player is OPPONENT
				score = if self.current_player == winning_player {0.0} else {1.0}
			},
			GameResult::Undecided => {
				if depth > 1 {
					let cache_key = self.compress();
					if score_cache.contains_key(&cache_key) {
						score = *score_cache.get(&cache_key).unwrap();
					} else {
						let mut max_score = 0.0;
						for option in self.get_all_options().iter() {
							let option_score = self.evaluate_move(&option, depth-1, score_cache);
							if option_score > max_score {
								max_score = option_score;
							}
							if max_score == 1.0 {
								break;
							}
						}
						score = 1.0 - max_score;
						score_cache.insert(cache_key, score);
					}
				} else {
					// score based on piece balance
					let balance = self.field.get_piece_balance(self.current_player);
					score = 0.5 - (balance as f64) / 10.0;
				}
			}
		};

		self.undo_move(option);
		score
	}

	fn count_options(&mut self, depth: usize) -> u64 {
		if depth == 0 {
			return 1;
		}
		if let GameResult::DecidedWithWinner(_) = self.get_result() {
			return 1;
		}
		let mut total_options: u64 = 0;
	    for option in self.get_all_options().iter() {
	    	self.make_move(option);
	    	total_options += self.count_options(depth-1);
	    	self.undo_move(option);
	    }
	    total_options
	}

	fn propose_evaluation_depth(&mut self, max_total_options: u64) -> usize {
		// check how many positions can be reached in 4 steps,
		// compute how many steps can be evaluated exhaustively.
		let options_per_level: f64 = (self.count_options(4) as f64).powf(0.25);
		let num_levels: usize = (max_total_options as f64).log(options_per_level).floor() as usize;
		if num_levels < 4 {
			return 4;
		}
		if num_levels > 12 {
			return 12;
		}
		num_levels
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
