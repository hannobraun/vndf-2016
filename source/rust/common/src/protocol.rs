pub mod client {
	#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
	pub enum Event {
		Public(event::Public),
		Privileged(event::Privileged),
	}


	pub mod event {
		#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
		pub enum Public {
			Login,
		}

		#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
		pub enum Privileged {
			Heartbeat,

			StartBroadcast(String),
			StopBroadcast,

			ScheduleManeuver(f32),
		}
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
