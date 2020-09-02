use std::fmt;
use crate::field::Field;
use crate::card::Card;


pub struct Game {
	pub field: Field,
	pub player_cards: ((Card, Card), (Card, Card)),
	pub public_card: Card,
	pub current_player: u8,
}

impl Game {
	pub fn new(cards: (Card, Card, Card, Card, Card)) -> Self {
		Self {
			field: Field::new(),
			player_cards: ((cards.0, cards.1), (cards.2, cards.3)),
			public_card: cards.4,
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
		write!(f, "\nPlayer 2: {}, {}\n\n", (self.player_cards.1).0, (self.player_cards.1).1)?;
		if self.current_player == 2 {
			write!(f, "Public: {}\n", self.public_card)?;
		}
		write!(f, "{}", self.field)?;
		if self.current_player == 1 {
			write!(f, "Public: {}\n", self.public_card)?;
		}
		write!(f, "Player 1: {}, {}\n\n", (self.player_cards.0).0, (self.player_cards.0).1)
	}
}
