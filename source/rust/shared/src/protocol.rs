pub mod client {
	#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
	pub enum Event {
		Public(event::Public),
		Privileged(event::Privileged),
	}

	impl Event {
		/// Returns whether the event should be considered important or not.
		/// This is currently only used to determine the log level of the event.
		pub fn is_important(&self) -> bool {
			use self::event::Public::*;
			use self::event::Privileged::*;

			match *self {
				Event::Public(Login)                   => true,
				Event::Privileged(Heartbeat)           => false,
				Event::Privileged(StartBroadcast(_))   => true,
				Event::Privileged(StopBroadcast)       => true,
				Event::Privileged(ScheduleManeuver(_)) => true,
			}
		}
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
	use game::{
		Broadcast,
		EntityId,
		Ship,
	};


	#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
	pub enum Event {
		Heartbeat,
		ShipId(EntityId),
		StartBroadcast(Broadcast),
		StopBroadcast(EntityId),
		UpdateShip(Ship),
	}
}
