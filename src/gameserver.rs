#![deny(warnings)]
use warp::Filter;
use std::path::Path;
use rusqlite::{params, Connection, Result};
use serde_json::json;

mod field;
mod piece;
mod position;
mod card;
mod game;
mod player;
mod move_option;

use game::{Game, GameType};

fn open_db() -> Connection {
	let db_path = Path::new("./games_db.sqlite");
    let conn = Connection::open(db_path).expect("Could not open DB connection");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (id INTEGER PRIMARY KEY, game TEXT NOT NULL)",
        params![],
    ).expect("Could not create table");
    conn
}

fn save_game(game_string: &str) -> Result<u32> {
    let conn = open_db();
    conn.execute(
        "INSERT INTO games (game) VALUES (?1)",
        params![game_string],
    )?;
    let game_id = conn.last_insert_rowid();
    Ok(game_id as u32)
}

fn list_game_ids() -> Result<Vec<u32>> {
    let conn = open_db();
    let mut statement = conn.prepare("SELECT id FROM games")?;
    let rows = statement.query_map(params![], |row| row.get(0))?;
    let mut result: Vec<u32> = Vec::new();
    for row in rows {
    	result.push(row?);
    }
    Ok(result)
}

fn load_game(game_id: u32) -> Result<String> {
    let conn = open_db();
    conn.query_row("SELECT game FROM games WHERE id = ?1", params![game_id], |row| row.get(0))
}

fn do_nonsense() -> String {
	let mut game = Game::new(GameType::HumanVsHuman);
    &game.run_turn();	
	let _game2 = Game::new(GameType::BotVsBot((1000000, 1000000)));
	"nonsense!".to_string()
}

fn create_game() -> String {
	let game = Game::new(GameType::HumanVsBot(1000000));
	let game_string = json!(game).to_string();
	match save_game(&game_string) {
		Ok(game_id) => { return json!(game_id).to_string(); },
		Err(_) => { return "An error ocurred".to_string(); },
	}
}

fn list_games() -> String {
	match list_game_ids() {
		Ok(game_ids) => { return json!(game_ids).to_string(); },
		Err(_) => { return "An error ocurred".to_string(); },
	}
}

fn get_game(game_id: u32) -> String {
	match load_game(game_id) {
		Ok(game_string) => { return game_string; },
		Err(_) => { return "An error ocurred".to_string(); },
	}
}

#[tokio::main]
async fn main() {
    let create_game = warp::post().and(warp::path!("api"/"games")).map(|| create_game());
    let list_games = warp::get().and(warp::path!("api"/"games")).map(|| list_games());
    let get_game = warp::get().and(warp::path!("api"/"games"/u32)).map(|game_id| get_game(game_id));

    // cannot have what rustc deems "dead code", although it is only dead in this entrypoint
    let _do_nonsense = warp::get().and(warp::path!("api"/"nonsense")).map(|| do_nonsense());

    // let get_options = warp::get().and(warp::path!("api"/"games"/u32/"options")).map(|_game_id| "Should get options");
    // let act = warp::put().and(warp::path!("api"/"games"/u32/"options"/u32)).map(|_game_id, _option_id| "Should act");

    let routes = create_game.or(list_games).or(get_game);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
