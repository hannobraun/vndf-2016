use shared::game::{
	EntityId,
	ManeuverData,
};


#[derive(Debug)]
pub struct Maneuver {
	pub ship_id: EntityId,
	pub data   : ManeuverData,
}
