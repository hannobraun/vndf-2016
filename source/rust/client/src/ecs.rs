use game::ecs::{
	Planet,
	Visual,
};
use game::physics::Body;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Interpolated {
	pub previous_time: u64,
	pub current_time : u64,

	pub previous: Option<Body>,
	pub current : Option<Body>
}

impl Interpolated {
	pub fn new(current_time: u64, body: Option<Body>) -> Interpolated {
		Interpolated {
			previous_time: current_time,
			current_time : current_time,

			previous: body,
			current : body
		}
	}
}


world!(
	entity_constructor craft(
		body        : Body,
		visual      : Visual,
		interpolated: Interpolated,
	) -> (Body, Visual, Interpolated) {
		(body, visual, interpolated)
	}

	entity_constructor planet(planet: Planet) -> (Planet) {
		(planet,)
	}
)
