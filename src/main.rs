mod field;
mod cell;
mod piece;
mod position;
mod card;
mod game;
mod player;

use rand::seq::SliceRandom;

use game::Game;
use card::Card;
use player::Player;

fn main() {

    let mut cards = Card::get_all_cards();
	let mut rng = &mut rand::thread_rng();	
	cards.shuffle(&mut rng);

	let players: [Player; 2] = [
		Player {name: "Player 1".to_string(), cards: [cards.pop().expect(""), cards.pop().expect("")]},
		Player {name: "Player 2".to_string(), cards: [cards.pop().expect(""), cards.pop().expect("")]},
	];

	let game = Game::new(players, cards.pop().expect(""));
    println!("{}", game);

    // let all_pieces = &field.get_all_pieces(true);
    // for (piece, position) in all_pieces.iter() {
	//     println!("Piece {} @ {}", piece, position);
	//     let all_moves = &field.get_all_moves(&piece, &position, &cards[0]);
	//     for target in all_moves.iter() {
	//     	println!("    Target {}", target);
	//     }
    // }
}
