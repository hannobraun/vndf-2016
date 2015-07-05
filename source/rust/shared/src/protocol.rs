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
				Event::Public(Login)                      => true,
				Event::Privileged(Heartbeat)              => false,
				Event::Privileged(StartBroadcast(_))      => true,
				Event::Privileged(StopBroadcast)          => true,
				Event::Privileged(ScheduleManeuver(_, _)) => true,
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

			ScheduleManeuver(f64, f64),
		}
	}


	pub fn login() -> Event {
		Event::Public(event::Public::Login)
	}

	pub fn start_broadcast(message: String) -> Event {
		Event::Privileged(event::Privileged::StartBroadcast(message))
	}

	pub fn schedule_maneuver(delay: f64, angle: f64) -> Event {
		Event::Privileged(event::Privileged::ScheduleManeuver(delay, angle))
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
		UpdateEntity(EntityId, (Ship, Option<Broadcast>)),
		RemoveEntity(EntityId),
	}
}
