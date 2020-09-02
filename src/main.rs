mod field;
mod cell;
mod piece;
mod position;
mod card;
mod game;
mod player;

use rand::seq::SliceRandom;

use game::Game;
use card::Card;
use player::Player;
use position::Position;

fn main() {

    let mut cards = Card::get_all_cards();
	let mut rng = &mut rand::thread_rng();	
	cards.shuffle(&mut rng);

	let players: [Player; 2] = [
		Player {name: "Blue player".to_string(), cards: [cards.pop().expect(""), cards.pop().expect("")]},
		Player {name: "Red player".to_string(), cards: [cards.pop().expect(""), cards.pop().expect("")]},
	];

	let game = Game::new(players, cards.pop().expect(""));
    println!("{}", game);

    let options: Vec<(Position, usize, Position)> = game.get_all_options();
    for (position, card_index, target_position) in options.iter() {
    	let card: &Card = &game.players[game.current_player].cards[*card_index];
	    println!("Option: {} -> {} (using {})", position, target_position, card);
    }
}
