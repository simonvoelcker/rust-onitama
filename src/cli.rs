mod field;
mod piece;
mod position;
mod card;
mod game;
mod player;
mod move_option;

use std::{env, process};
use game::{Game, GameType, GameResult};

fn int_or_bust(s: &String) -> u64 {
	match s.parse() {
		Ok(i) => { i },
		Err(_) => {
			println!("Integer expected.");
			process::exit(1);
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut game = match args.len() {
		1 => {
			Game::new(GameType::HumanVsHuman)
		},
		2 => {
			let strength = int_or_bust(&args[1]);
			Game::new(GameType::HumanVsBot(strength))
		},
		3 => {
			let strength1 = int_or_bust(&args[1]);
			let strength2 = int_or_bust(&args[2]);
			Game::new(GameType::BotVsBot((strength1, strength2)))
		},
		_ => {
			println!("Bad arguments.");
			process::exit(1);
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