use common::ecs::Components;
use common::physics::Body;

use game::data::Ship;


pub struct Entities {
	pub bodies: Components<Body>,
	pub ships : Components<Ship>
}
