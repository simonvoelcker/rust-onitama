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

use api::{create_game, list_games, get_game, list_options, pick_option, do_nonsense};


#[tokio::main]
async fn main() {

    // cannot have what rustc deems "dead code", although it is only dead in this entrypoint
    if false {
    	do_nonsense();
    }

    let routes = warp::post().and(warp::path!("api"/"games")).map(create_game)
    	.or(warp::get().and(warp::path!("api"/"games")).map(list_games))
    	.or(warp::get().and(warp::path!("api"/"games"/u32)).map(get_game))
    	.or(warp::get().and(warp::path!("api"/"games"/u32/"options")).map(list_options))
    	.or(warp::post().and(warp::path!("api"/"games"/u32/"options"/u32)).map(pick_option));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
