use std::f64;
use std::libc;
use std::ptr;

use clients::Clients;

extern {
	fn close(fd: libc::c_int) -> libc::c_int;
}


pub static ON_CONNECT   : uint = 0;
pub static ON_DISCONNECT: uint = 1;
pub static ON_UPDATE    : uint = 2;

pub enum Event {
	Connect(libc::c_int),
	Disconnect(uint),
	Update
}

pub struct Events {
	first : u64,
	last  : u64,
	cap   : libc::size_t,
	buffer: *mut Event
}


pub fn handle_events(events: &mut Events, clients: &mut Clients, frameTimeInMs: libc::c_int) {
	unsafe {
		while (events.last - events.first > 0) {
			let event = *(ptr::mut_offset(events.buffer, (events.first % events.cap) as int));
			events.first += 1;

			match event {
				Connect(clientFD)    => on_connect(clientFD, clients),
				Disconnect(clientId) => on_disconnect(clientId, clients, events),
				Update              => on_update(clients, events, frameTimeInMs as f64 / 1000.0),
			}
		}
	}
}

fn on_connect(clientFD: libc::c_int, clients: &mut Clients) {
	let distance = 100.0;

	let alpha = 90.0 / 180.0 * f64::consts::PI;

	let pos = ::common::vec::Vec2 {
		x: distance * f64::cos(alpha),
		y: distance * f64::sin(alpha) };

	let vel = ::common::vec::Vec2 {
		x: 30.0,
		y: 0.0 };

	if !clients.add(clientFD, pos, vel) {
		unsafe {
			close(clientFD);
		}
	}
}

fn on_disconnect(clientId: uint, clients: &mut Clients, events: &mut Events) {
	clients.remove(clientId);

	clients.each(|client| {
		let status = ::protocol::send_remove(
			client.socketFD,
			clientId);

		if (status < 0) {
			unsafe {
				let ptr = ptr::mut_offset(events.buffer, (events.last % events.cap) as int);
				*ptr = Disconnect(client.id);
				events.last += 1;
			}
		}
	})
}

fn on_update(clients: &mut Clients, events: &mut Events, dTimeInS: f64) {
	clients.mut_each(|client| {
		let gMag = 3000.0 / client.ship.pos.magnitude();
		let g = client.ship.pos.normalize() * -gMag;

		client.ship.pos = client.ship.pos + client.ship.vel * dTimeInS;
		client.ship.vel = client.ship.vel + g * dTimeInS;
	});

	clients.each(|clientA| {
		clients.each(|clientB| {
			let status = ::protocol::send_update(
				clientA.socketFD,
				clientB.id,
				clientB.ship.pos.x,
				clientB.ship.pos.y);

			if (status < 0) {
				unsafe {
					let ptr = ptr::mut_offset(events.buffer, (events.last % events.cap) as int);
					*ptr = Disconnect(clientA.id);
				}
				events.last += 1;
			}
		})
	});
}
