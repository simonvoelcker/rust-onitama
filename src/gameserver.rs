#![deny(warnings)]
use warp::Filter;

mod api;
mod db;

mod field;
mod piece;
mod position;
mod card;
mod game;
mod player;
mod move_option;

use api::{create_game, list_games, get_game, list_options, do_nonsense};


#[tokio::main]
async fn main() {
    let create_game = warp::post().and(warp::path!("api"/"games")).map(|| create_game());
    let list_games = warp::get().and(warp::path!("api"/"games")).map(|| list_games());
    let get_game = warp::get().and(warp::path!("api"/"games"/u32)).map(|game_id| get_game(game_id));
    let list_options = warp::get().and(warp::path!("api"/"games"/u32/"options")).map(|game_id| list_options(game_id));

    // cannot have what rustc deems "dead code", although it is only dead in this entrypoint
    if false {
    	do_nonsense();
    }

    // let act = warp::put().and(warp::path!("api"/"games"/u32/"options"/u32)).map(|_game_id, _option_id| "Should act");

    let routes = create_game.or(list_games).or(get_game).or(list_options);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
