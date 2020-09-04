mod field;
mod piece;
mod position;
mod card;
mod game;
mod player;
mod move_option;

use std::io;
use std::io::Write;
use rand::seq::SliceRandom;

use game::Game;
use card::Card;
use player::Player;
use move_option::MoveOption;

fn main() {

    let mut cards = Card::get_all_cards();
	let mut rng = &mut rand::thread_rng();	
	cards.shuffle(&mut rng);

	let players: [Player; 2] = [
		Player {name: "Blue player".to_string(), cards: [cards.pop().expect(""), cards.pop().expect("")]},
		Player {name: "Red player".to_string(), cards: [cards.pop().expect(""), cards.pop().expect("")]},
	];

	let mut game = Game::new(players, cards.pop().expect(""));

	while !game.is_over() {
	    println!("{}", game);

	    let options: Vec<MoveOption> = game.get_all_options();
	    for (option_index, option) in options.iter().enumerate() {
		    println!("Option {:2}: {}", option_index+1, option);
	    }

	    let mut choice = 0;
	    while choice < 1 || choice > options.len() {
		    print!("Choose option: ");
		    io::stdout().flush().unwrap();
		    let mut input = String::new();
		    io::stdin().read_line(&mut input).unwrap();
		    match input.trim().parse() {
		    	Ok(num) => {choice = num},
		    	Err(_) => {choice = 0},
		    };
	    }

	    game.make_move(&options[choice-1]);
	}
}
