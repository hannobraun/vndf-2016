extern mod common;
extern mod extra;

pub mod clients;
pub mod events;
pub mod net;
pub mod protocol;
pub mod util;


extern {
	fn time(timer: ::std::libc::c_uint) -> ::std::libc::c_uint;
	fn srand(seed: ::std::libc::c_uint);
}


fn main() {
	util::log("Core Service started.");

	unsafe {
		srand(time(0));

		let net = net::init("34481");

		let mut events = events::Events {
			first : 0,
			last  : 0,
			cap   : 16,
			buffer: ::std::libc::malloc(16 * ::std::mem::size_of::<events::Event>() as u64) as *mut events::Event};

		let mut clientMap = clients::init_client_map(4);

		loop {
			let frameTimeInMs = 50;
			let numberOfEvents= net::number_of_events(&net, frameTimeInMs) as int;
			handle_connects(numberOfEvents, net.serverFD, &mut events);
			schedule_update(&mut events);
			events::handle_events(&mut events, clientMap, frameTimeInMs);
		}
	}
}

fn handle_connects(numberOfEvents: int, serverFD: ::std::libc::c_int, events: &mut events::Events) {
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

fn schedule_update(events: &mut events::Events) {
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
