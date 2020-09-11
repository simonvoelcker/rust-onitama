#![deny(warnings)]
use warp::Filter;
use std::path::Path;

// mod field;
// mod piece;
// mod position;
// mod card;
// mod game;
// mod player;
// mod move_option;

use rusqlite::{params, Connection, Result};
// use game::{Game, GameType};

fn save_game(game_string: &str) -> Result<i64> {
	let db_path = Path::new("./games_db.sqlite");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (id INTEGER PRIMARY KEY, game TEXT NOT NULL)",
        params![],
    )?;
    conn.execute(
        "INSERT INTO games (game) VALUES (?1)",
        params![game_string],
    )?;
    let game_id = conn.last_insert_rowid();
    Ok(game_id)
}

//fn load_game(_game_id: u64) -> Result<()> {
    // let conn = Connection::open_in_memory()?;
    // conn.execute(
    //     "CREATE TABLE IF NOT EXISTS games (id INTEGER PRIMARY KEY, game TEXT NOT NULL)",
    //     params![],
    // )?;
    // let mut stmt = conn.prepare("SELECT name FROM games WHERE id = ?1")?;
    // let game_iter = stmt.query_map(params!["<game_id>"], |row| { row.get(0) })?;
//    Ok(())
//}

fn create_game() -> String {
	// let game = Game::new(GameType::HumanVsBot(1000000));
	match save_game(&"my-game-string") {
		Ok(game_id) => { return format!("Created game with id {}", game_id); },
		Err(_) => { return "An error ocurred".to_string(); },
	}
}

#[tokio::main]
async fn main() {
    let create_game = warp::post().and(warp::path!("api"/"games")).map(|| create_game());

    let get_game = warp::get().and(warp::path!("api"/"games"/u32)).map(|game_id| format!("Should get game with id {}", game_id));
    let get_options = warp::get().and(warp::path!("api"/"games"/u32/"options")).map(|_game_id| "Should get options");
    let act = warp::put().and(warp::path!("api"/"games"/u32/"options"/u32)).map(|_game_id, _option_id| "Should act");

    let routes = create_game.or(get_game).or(get_options).or(act);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
