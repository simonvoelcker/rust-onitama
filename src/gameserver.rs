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

    let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET", "POST"]);

    let api_routes = warp::post().and(warp::path!("api"/"games")).map(create_game)
    	.or(warp::get().and(warp::path!("api"/"games")).map(list_games))
    	.or(warp::get().and(warp::path!("api"/"games"/u32)).map(get_game))
    	.or(warp::get().and(warp::path!("api"/"games"/u32/"options")).map(list_options))
    	.or(warp::post().and(warp::path!("api"/"games"/u32/"options"/u32)).map(pick_option));

    let client_route = warp::get().and(warp::fs::dir("client/build"));
    let img_route = warp::get().and(warp::path("img")).and(warp::fs::dir("client/img"));

    let routes = api_routes.or(client_route).or(img_route).with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}
