mod field;
mod cell;
mod piece;

use field::Field;

fn main() {
	let field = Field::new();
    println!("{}", field);
}
