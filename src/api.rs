use serde_json::json;
use crate::game::{Game, GameType};
use crate::db::{save_game, list_game_ids, load_game};

pub fn create_game() -> String {
	let game = Game::new(GameType::HumanVsBot(1000000));
	let game_string = json!(game).to_string();
	match save_game(&game_string) {
		Ok(game_id) => { return json!(game_id).to_string(); },
		Err(_) => { return "An error ocurred".to_string(); },
	}
}

pub fn list_games() -> String {
	match list_game_ids() {
		Ok(game_ids) => { return json!(game_ids).to_string(); },
		Err(_) => { return "An error ocurred".to_string(); },
	}
}

pub fn get_game(game_id: u32) -> String {
	match load_game(game_id) {
		Ok(game_string) => { return game_string; },
		Err(_) => { return "An error ocurred".to_string(); },
	}
}

pub fn list_options(game_id: u32) -> String {
	match load_game(game_id) {
		Ok(_game_string) => {
			// let game: Game = from_str(&game_string).unwrap();
			// let options: Vec<MoveOption> = game.get_all_options();
			// return json!(options).to_string();
			return "".to_string();
		},
		Err(_) => { return "An error ocurred".to_string(); },
	}
}

pub fn do_nonsense() -> String {
	let mut game = Game::new(GameType::HumanVsHuman);
    &game.run_turn();	
	let _game2 = Game::new(GameType::BotVsBot((1000000, 1000000)));
	"nonsense!".to_string()
}
