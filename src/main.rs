mod field;
mod cell;
mod piece;

use field::Field;

fn main() {
	let field = Field::new();
    println!("{}", field);

    let all_pieces = field.get_all_pieces(true);
    for (piece, x, y) in all_pieces.iter() {
	    println!("{} @ {},{}", piece, x, y);
    }
}
