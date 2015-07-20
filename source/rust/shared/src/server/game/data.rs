use shared::game::EntityId;


#[derive(Debug)]
pub struct Maneuver {
	pub ship_id: EntityId,
	pub start_s: f64,
	pub angle  : f64,
}
