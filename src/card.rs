use std::fmt;
use crate::position::Offset;

#[derive(Clone)]
pub struct Card {
	pub name: &'static str,
	pub color: &'static str,
	pub moves: [Offset; 4],
}

impl Card {
	pub const fn get_all_cards() -> [Card; 16] {
		return [
			Card {
				name: "Affe",
				color: "Blue",
				moves: [Offset {x:-1,y:1}, Offset {x:1,y:1}, Offset {x:-1,y:-1}, Offset {x:1,y:-1}]
			},
			Card {
				name: "Drache",
				color: "Red",
				moves: [Offset {x:-2,y:1}, Offset {x:2,y:1}, Offset {x:-1,y:-1}, Offset {x:1,y:-1}]
			},
			Card {
				name: "Elefant",
				color: "Red",
				moves: [Offset {x:-1,y:1}, Offset {x:1,y:1}, Offset {x:-1,y:0}, Offset {x:1,y:0}]
			},
			Card {
				name: "Krabbe",
				color: "Blue",
				moves: [Offset {x:0,y:1}, Offset {x:-2,y:0}, Offset {x:2,y:0}, 			Offset {x:2,y:0}]
			},
			Card {
				name: "Tiger",
				color: "Blue",
				moves: [Offset {x:0,y:2}, Offset {x:0,y:-1}, 							Offset {x:0,y:-1}, Offset {x:0,y:-1}]
			},
			Card {
				name: "Gans",
				color: "Blue",
				moves: [Offset {x:-1,y:1}, Offset {x:-1,y:0}, Offset {x:1,y:0}, Offset {x:1,y:-1}]
			},
			Card {
				name: "Hahn",
				color: "Red",
				moves: [Offset {x:1,y:1}, Offset {x:-1,y:0}, Offset {x:1,y:0}, Offset {x:-1,y:-1}]
			},
			Card {
				name: "Ochse",
				color: "Blue",
				moves: [Offset {x:0,y:1}, Offset {x:1,y:0}, Offset {x:0,y:-1}, 						Offset {x:0,y:-1}]
			},
			Card {
				name: "Pferd",
				color: "Red",
				moves: [Offset {x:0,y:1}, Offset {x:-1,y:0}, Offset {x:0,y:-1}, 					Offset {x:0,y:-1}]
			},
			Card {
				name: "Wildschwein",
				color: "Red",
				moves: [Offset {x:0,y:1}, Offset {x:-1,y:0}, Offset {x:1,y:0}, 						Offset {x:1,y:0}]
			},
			Card {
				name: "Aal",
				color: "Blue",
				moves: [Offset {x:-1,y:1}, Offset {x:1,y:0}, Offset {x:-1,y:-1}, 					Offset {x:-1,y:-1}]
			},
			Card {
				name: "Gottesanbeterin",
				color: "Red",
				moves: [Offset {x:-1,y:1}, Offset {x:1,y:1}, Offset {x:0,y:-1}, 					Offset {x:0,y:-1}]
			},
			Card {
				name: "Kobra",
				color: "Red",
				moves: [Offset {x:1,y:1}, Offset {x:-1,y:0}, Offset {x:1,y:-1}, 					Offset {x:1,y:-1}]
			},
			Card {
				name: "Kranich",
				color: "Blue",
				moves: [Offset {x:0,y:1}, Offset {x:-1,y:-1}, Offset {x:1,y:-1}, 					Offset {x:1,y:-1}]
			},
			Card {
				name: "Frosch",
				color: "Red",
				moves: [Offset {x:-1,y:1}, Offset {x:-2,y:0}, Offset {x:1,y:-1}, 					Offset {x:1,y:-1}]
			},
			Card {
				name: "Hase",
				color: "Blue",
				moves: [Offset {x:1,y:1}, Offset {x:2,y:0}, Offset {x:-1,y:-1}, 					Offset {x:-1,y:-1}]
			},
		];
	}
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}

impl std::cmp::PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
