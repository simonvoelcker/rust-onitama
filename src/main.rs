mod field;
mod cell;
mod piece;
mod position;
mod card;

use field::Field;
use card::Card;

fn main() {
	let field = Field::new();
    println!("{}", field);

    let cards = Card::get_all_cards();

    let all_pieces = &field.get_all_pieces(true);
    for (piece, position) in all_pieces.iter() {
	    println!("Piece {} @ {}", piece, position);
	    let all_moves = &field.get_all_moves(&piece, &position, &cards[0]);
	    for target in all_moves.iter() {
	    	println!("    Target {}", target);
	    }
    }
}
