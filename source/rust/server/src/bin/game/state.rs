use game::entities::Entities;


#[derive(Debug)]
pub struct GameState {
	pub entities: Entities,
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			entities: Entities::new(),
		}
	}
}
