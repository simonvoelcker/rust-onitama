use std::path::Path;
use rusqlite::{params, Connection, Result};


fn open_db() -> Connection {
	let db_path = Path::new("./games_db.sqlite");
    let conn = Connection::open(db_path).expect("Could not open DB connection");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (id INTEGER PRIMARY KEY, game TEXT NOT NULL)",
        params![],
    ).expect("Could not create table");
    conn
}

pub fn create_game_entry(game_string: &str) -> Result<u32> {
    let conn = open_db();
    conn.execute(
        "INSERT INTO games (game) VALUES (?1)",
        params![game_string],
    )?;
    let game_id = conn.last_insert_rowid();
    Ok(game_id as u32)
}

pub fn list_game_ids() -> Result<Vec<u32>> {
    let conn = open_db();
    let mut statement = conn.prepare("SELECT id FROM games")?;
    let rows = statement.query_map(params![], |row| row.get(0))?;
    let mut result: Vec<u32> = Vec::new();
    for row in rows {
    	result.push(row?);
    }
    Ok(result)
}

pub fn load_game(game_id: u32) -> Result<String> {
    let conn = open_db();
    conn.query_row("SELECT game FROM games WHERE id = ?1", params![game_id], |row| row.get(0))
}

pub fn save_game(game_id: u32, game_string: &str) -> Result<()> {
    let conn = open_db();
    conn.execute(
        "UPDATE games SET game = ?1 WHERE id = ?2",
        params![game_string, game_id],
    )?;
    Ok(())
}
