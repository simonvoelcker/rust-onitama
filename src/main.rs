mod field;
mod cell;
mod piece;
mod position;

use field::Field;

fn main() {
	let field = Field::new();
    println!("{}", field);

    let all_pieces = field.get_all_pieces(true);
    for (piece, position) in all_pieces.iter() {
	    println!("{} @ {}", piece, position);
    }
}
