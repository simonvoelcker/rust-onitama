use std::fmt;
use crate::position::Offset;

pub struct Card {
	pub name: String,
	pub moves: Vec<Offset>
}

impl Card {
	pub fn get_all_cards() -> Vec<Card> {
		return vec![
			Card {
				name: "Affe".to_string(),
				moves: vec![Offset {x:-1,y:1}, Offset {x:1,y:1}, Offset {x:-1,y:-1}, Offset {x:1,y:-1}]
			},
			Card {
				name: "Drache".to_string(),
				moves: vec![Offset {x:-2,y:1}, Offset {x:2,y:1}, Offset {x:-1,y:-1}, Offset {x:1,y:-1}]
			},
			Card {
				name: "Elefant".to_string(),
				moves: vec![Offset {x:-1,y:1}, Offset {x:1,y:1}, Offset {x:-1,y:0}, Offset {x:1,y:0}]
			},
			Card {
				name: "Krabbe".to_string(),
				moves: vec![Offset {x:0,y:1}, Offset {x:-2,y:0}, Offset {x:2,y:0}]
			},
			Card {
				name: "Tiger".to_string(),
				moves: vec![Offset {x:0,y:2}, Offset {x:0,y:-1}]
			},
			Card {
				name: "Gans".to_string(),
				moves: vec![Offset {x:-1,y:1}, Offset {x:-1,y:0}, Offset {x:1,y:0}, Offset {x:1,y:-1}]
			},
			Card {
				name: "Hahn".to_string(),
				moves: vec![Offset {x:1,y:1}, Offset {x:-1,y:0}, Offset {x:1,y:0}, Offset {x:-1,y:-1}]
			},
			Card {
				name: "Ochse".to_string(),
				moves: vec![Offset {x:0,y:1}, Offset {x:1,y:0}, Offset {x:0,y:-1}]
			},
			Card {
				name: "Pferd".to_string(),
				moves: vec![Offset {x:0,y:1}, Offset {x:-1,y:0}, Offset {x:0,y:-1}]
			},
			Card {
				name: "Wildschwein".to_string(),
				moves: vec![Offset {x:0,y:1}, Offset {x:-1,y:0}, Offset {x:1,y:0}]
			},
			Card {
				name: "Aal".to_string(),
				moves: vec![Offset {x:-1,y:1}, Offset {x:1,y:0}, Offset {x:-1,y:-1}]
			},
			Card {
				name: "Gottesanbeterin".to_string(),
				moves: vec![Offset {x:-1,y:1}, Offset {x:1,y:1}, Offset {x:0,y:-1}]
			},
			Card {
				name: "Kobra".to_string(),
				moves: vec![Offset {x:1,y:1}, Offset {x:-1,y:0}, Offset {x:1,y:-1}]
			},
			Card {
				name: "Kranich".to_string(),
				moves: vec![Offset {x:0,y:1}, Offset {x:-1,y:-1}, Offset {x:1,y:-1}]
			},
			Card {
				name: "Frosch".to_string(),
				moves: vec![Offset {x:-1,y:1}, Offset {x:-2,y:0}, Offset {x:1,y:-1}]
			},
			Card {
				name: "Hase".to_string(),
				moves: vec![Offset {x:1,y:1}, Offset {x:2,y:0}, Offset {x:-1,y:-1}]
			},
		];
	}
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}
