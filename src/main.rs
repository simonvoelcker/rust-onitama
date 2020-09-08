mod field;
mod piece;
mod position;
mod card;
mod game;
mod player;
mod move_option;

use rand::seq::SliceRandom;

use game::{Game, GameResult};
use card::Card;
use player::Player;

static CARDS: [Card; 16] = Card::get_all_cards();

fn main() {

	let mut rng = &mut rand::thread_rng();
	let mut indices: Vec<usize> = (0..16).collect();
	indices.shuffle(&mut rng);

	let player1_cards = [&CARDS[indices[0]], &CARDS[indices[1]]];
	let player2_cards = [&CARDS[indices[2]], &CARDS[indices[3]]];
	let public_card = &CARDS[indices[4]];

	let players: [Player; 2] = [
		Player {name: "Blue player", cards: player1_cards, is_bot: true},
		Player {name: "Red player", cards: player2_cards, is_bot: true},
	];

	let mut game = Game::new(players, public_card);

	loop {
	    println!("{}", game);
	    &game.run_turn();

	    if let GameResult::DecidedWithWinner(_) = game.get_result() {
	    	println!("The game is over!");
		    println!("{}", game);
	    	break;
	    }
	}
}