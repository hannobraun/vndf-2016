extern mod common;
extern mod extra;

use clients::Clients;

pub mod clients;
pub mod events;
pub mod net;
pub mod protocol;
pub mod util;


fn main() {
	util::log("Core Service started.");

	unsafe {
		let net = net::init("34481");

		let mut events = events::Events {
			first : 0,
			last  : 0,
			cap   : 16,
			buffer: ::std::libc::malloc(16 * ::std::mem::size_of::<events::Event>() as u64) as *mut events::Event};

		let mut clientMap = Clients::new(4);

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

		unsafe {
			*(::std::ptr::mut_offset(events.buffer, (events.last % events.cap) as int)) = events::Connect(clientFD);
			events.last += 1;
		}

		i += 1;
	}
}

fn schedule_update(events: &mut events::Events) {
	unsafe {
		*(::std::ptr::mut_offset(events.buffer, (events.last % events.cap) as int)) = events::Update;
		events.last += 1;
	}
}
