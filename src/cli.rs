mod field;
mod piece;
mod position;
mod card;
mod game;
mod player;
mod move_option;

use std::env;
use game::{Game, GameType, GameResult};

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut game = match args.len() {
		1 => {
			Game::new(GameType::HumanVsHuman)
		},
		2 => {
			let strength = args[1].parse().expect("Integer expected");
			Game::new(GameType::HumanVsBot(strength))
		},
		3 => {
			let strength1 = args[1].parse().expect("Integer expected");
			let strength2 = args[2].parse().expect("Integer expected");
			Game::new(GameType::BotVsBot((strength1, strength2)))
		},
		_ => {
			panic!("Unexpected arguments. Provide at most 2 integers.")
		}
	};

	loop {
	    if let GameResult::DecidedWithWinner(_) = game.get_result() {
	    	println!("The game is over!");
		    println!("{}", game);
	    	break;
	    }

	    println!("{}", game);
	    &game.run_turn();
	}
}