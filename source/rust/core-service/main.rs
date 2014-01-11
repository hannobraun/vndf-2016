#[crate_type = "rlib"];
#[crate_type = "staticlib"];
#[crate_id = "core-service"];


extern mod common;
extern mod extra;

pub mod clients;
pub mod events;
pub mod net;
pub mod protocol;
pub mod util;


#[no_mangle]
pub extern fn handle_connects(numberOfEvents: int, serverFD: ::std::libc::c_int, events: &mut events::Events) {
	let mut i = 0;
	while i < numberOfEvents {
		let clientFD = net::accept_client(serverFD);

		let event = events::Event {
			theType: events::ON_CONNECT,

			onConnect: events::ConnectEvent {
				clientFD: clientFD },
			onDisconnect: events::DisconnectEvent {
				clientId: 0 },
			onUpdate: events::UpdateEvent {
				dummy: 0 } };

		unsafe {
			*(::std::ptr::mut_offset(events.buffer, (events.last % events.cap) as int)) = event;
			events.last += 1;
		}

		i += 1;
	}
}

#[no_mangle]
pub extern fn schedule_update(events: &mut events::Events) {
	let event = events::Event {
		theType: events::ON_UPDATE,

		onConnect: events::ConnectEvent {
			clientFD: 0 },
		onDisconnect: events::DisconnectEvent {
			clientId: 0 },
		onUpdate: events::UpdateEvent {
			dummy: 0 } };

	unsafe {
		*(::std::ptr::mut_offset(events.buffer, (events.last % events.cap) as int)) = event;
		events.last += 1;
	}
}
