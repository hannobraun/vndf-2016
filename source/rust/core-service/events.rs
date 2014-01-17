use std::f64;
use std::libc;
use std::ptr;

use clients::Clients;

extern {
	fn close(fd: libc::c_int) -> libc::c_int;
}


pub struct Events {
	first : u64,
	last  : u64,
	cap   : libc::size_t,
	buffer: *mut Event
}

pub enum Event {
	Connect(libc::c_int),
	Disconnect(uint),
	Update
}

impl Events {
	pub fn push(&mut self, event: Event) {
		unsafe {
			let ptr = ptr::mut_offset(self.buffer, (self.last % self.cap) as int);
			*ptr = event;
			self.last += 1;
		}
	}

	pub fn pull(&mut self) -> Option<Event> {
		unsafe {
			if self.last - self.first > 0 {
				let event = *(ptr::mut_offset(self.buffer, (self.first % self.cap) as int));
				self.first += 1;

				Some(event)
			}
			else {
				None
			}
		}
	}
}


pub fn handle_events(events: &mut Events, clients: &mut Clients, frameTimeInMs: libc::c_int) {
	loop {
		match events.pull() {
			Some(event) =>
				match event {
					Connect(clientFD)    => on_connect(clientFD, clients),
					Disconnect(clientId) => on_disconnect(clientId, clients, events),
					Update               => on_update(clients, events, frameTimeInMs as f64 / 1000.0)
				},

			None => break
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
			events.push(Disconnect(client.id));
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
				events.push(Disconnect(clientA.id));
			}
		})
	});
}
