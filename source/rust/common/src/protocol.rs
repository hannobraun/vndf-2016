pub mod client {
	#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
	// TODO(720RWYSw): Split events into public and restricted events. This can
	//                 make the server-side event handling more robust.
	pub enum Event {
		Login,
		Heartbeat,

		StartBroadcast(String),
		StopBroadcast,

		ScheduleManeuver(f32),
	}
}


pub mod server {
	use nalgebra::Vec2;

	use game::Broadcast;


	#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
	pub enum Event {
		Heartbeat,
		SelfId(String),
		StartBroadcast(Broadcast),
		StopBroadcast(String),
		UpdateEntity(Vec2<f64>, Vec2<f64>),
	}
}
