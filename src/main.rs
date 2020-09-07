mod field;
mod piece;
mod position;
mod card;
mod game;
mod player;
mod move_option;

use std::io;
use std::io::Write;
use std::collections::HashMap;
use rand::seq::SliceRandom;

use game::{Game, GameResult};
use card::Card;
use player::Player;
use move_option::MoveOption;

static CARDS: [Card; 16] = Card::get_all_cards();

fn main() {

	let mut rng = &mut rand::thread_rng();
	let mut indices: Vec<usize> = (0..16).collect();
	indices.shuffle(&mut rng);

	let player1_cards = [&CARDS[indices[0]], &CARDS[indices[1]]];
	let player2_cards = [&CARDS[indices[2]], &CARDS[indices[3]]];
	let public_card = &CARDS[indices[4]];

	let players: [Player; 2] = [
		Player {name: "Blue player", cards: player1_cards},
		Player {name: "Red player", cards: player2_cards},
	];

	let mut game = Game::new(players, public_card);

	loop {
	    println!("{}", game);
	    let options: Vec<MoveOption> = game.get_all_options();

    	if game.current_player == 1 {
			let mut score_cache: HashMap<u64, f64> = HashMap::new();
			let mut highest_score: f64 = 0.0;
			let mut best_option_index: usize = 0;
			print!("Thinking about {} possible moves", options.len());
		    io::stdout().flush().unwrap();
		    for (option_index, option) in options.iter().enumerate() {
		    	let score = game.evaluate_move(&option, 7, &mut score_cache);
		    	if score > highest_score {
		    		highest_score = score;
		    		best_option_index = option_index;
		    	}
		    	print!(".");
			    io::stdout().flush().unwrap();
		    }
		    println!("\nBot's move: {} (Score is {})", options[best_option_index], highest_score);
		    game.make_move(&options[best_option_index]);
		} else {
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

	    if let GameResult::DecidedWithWinner(winning_player) = game.get_result() {
	    	println!("Player {} won the game!", winning_player);
	    	break;
	    }
	}
}