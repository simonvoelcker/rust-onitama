mod field;
mod cell;
mod piece;

use field::Field;

fn main() {
	let field = Field::new();
    println!("{}", field);

    let all_pieces = field.get_all_pieces();
    for piece in all_pieces.iter() {
	    println!("{}", piece);
    }
}
