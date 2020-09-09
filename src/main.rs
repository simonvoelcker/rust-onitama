mod field;
mod piece;
mod position;
mod card;
mod game;
mod player;
mod move_option;

use game::{Game, GameType, GameResult};

fn main() {
	let mut game = Game::new(GameType::HumanVsBot(3000000));

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