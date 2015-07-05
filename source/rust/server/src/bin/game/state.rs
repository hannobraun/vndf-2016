use game::entities::Entities;


#[derive(Debug)]
pub struct GameState {
	// TODO: Make entities private
	pub entities: Entities,
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			entities: Entities::new(),
		}
	}
}
