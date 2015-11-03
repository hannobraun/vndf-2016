use shared::game::data::{
	EntityId,
	ManeuverData,
};


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
			Event::Privileged(CancelManeuver(_))   => true,
			Event::Privileged(FtlJump(_))          => true,
		}
	}
}


pub mod event {
	use shared::game::data::{
		EntityId,
		ManeuverData,
	};


	#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
	pub enum Public {
		Login,
	}

	#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
	pub enum Privileged {
		Heartbeat,

		StartBroadcast(String),
		StopBroadcast,

		ScheduleManeuver(ManeuverData),
		CancelManeuver(EntityId),

		FtlJump(f64),
	}
}


pub fn login() -> Event {
	Event::Public(event::Public::Login)
}

pub fn start_broadcast(message: String) -> Event {
	Event::Privileged(event::Privileged::StartBroadcast(message))
}

pub fn schedule_maneuver(data: ManeuverData) -> Event {
	Event::Privileged(event::Privileged::ScheduleManeuver(data))
}

pub fn cancel_maneuver(id: EntityId) -> Event {
	Event::Privileged(event::Privileged::CancelManeuver(id))
}

pub fn ftl_jump(destination_time_s: f64) -> Event {
	Event::Privileged(event::Privileged::FtlJump(destination_time_s))
}
