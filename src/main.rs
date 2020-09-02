mod field;
mod cell;
mod piece;
mod position;
mod card;
mod game;

use game::Game;
use card::Card;

fn main() {

    let mut cards = Card::get_all_cards();
	let game = Game::new((
		cards.pop().expect(""),
		cards.pop().expect(""),
		cards.pop().expect(""),
		cards.pop().expect(""),
		cards.pop().expect(""),
	));
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
