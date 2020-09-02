use std::fmt;
use crate::field::Field;
use crate::card::Card;
use crate::player::Player;


pub struct Game {
	pub field: Field,
	pub players: (Player, Player),
	pub public_card: Card,
	pub current_player: u8,
}

impl Game {
	pub fn new(players: (Player, Player), public_card: Card) -> Self {
		Self {
			field: Field::new(),
			players,
			public_card,
			current_player: 1,
		}
	}

	// pub fn get_all_moves(&self, player: u8) -> Vec<Position> {
	// 	let mut targets: Vec<Position> = Vec::new();
	// 	for offset in card.moves.iter() {
	// 		let target = position.offset(&offset);
	// 		if target.in_field() {
	// 			targets.push(target)
	// 		}
	// 	}
	// 	targets
	// }
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\n{}\n", self.players.1)?;
		if self.current_player == 2 {
			write!(f, "Public: {}\n", self.public_card)?;
		}
		write!(f, "{}", self.field)?;
		if self.current_player == 1 {
			write!(f, "Public: {}\n", self.public_card)?;
		}
		write!(f, "{}\n", self.players.0)
	}
}
