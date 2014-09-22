use game::ecs::Visual;
use physics::Body;


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


ecs!(
	entity(Craft<Body, Visual, Interpolated>): |body: Body, visual: Visual, current_time: u64| {
		(
			body,
			visual,
			Interpolated::new(current_time, Some(body))
		)
	}

	world(World<Craft>)
)
