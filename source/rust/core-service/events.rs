extern {
	fn close(fd: ::std::libc::c_int) -> ::std::libc::c_int;
}


static ON_CONNECT: ::std::libc::c_int    = 0;
static ON_DISCONNECT: ::std::libc::c_int = 1;
static ON_UPDATE: ::std::libc::c_int     = 2;

struct Event {
	theType: ::std::libc::c_int,

	onConnect   : ConnectEvent,
	onDisconnect: DisconnectEvent,
	onUpdate    : UpdateEvent
}

struct ConnectEvent {
	clientFD: ::std::libc::c_int
}

struct DisconnectEvent {
	clientId: ::std::libc::size_t
}

struct UpdateEvent {
	dummy: ::std::libc::c_int
}

struct Events {
	first : u64,
	last  : u64,
	cap   : ::std::libc::size_t,
	buffer: *mut Event
}


#[no_mangle]
pub extern fn handle_events(events: &mut Events, clientMap: &mut ::clients::ClientMap, frameTimeInMs: ::std::libc::c_int) {
	unsafe {
		while (events.last - events.first > 0) {
			let event = *(::std::ptr::mut_offset(events.buffer, (events.first % events.cap) as int));
			events.first += 1;

			match event.theType {
				ON_CONNECT    => onConnect(event.onConnect.clientFD, clientMap),
				ON_DISCONNECT => onDisconnect(event.onDisconnect.clientId, clientMap, events),
				ON_UPDATE     => onUpdate(clientMap, events, frameTimeInMs as f64 / 1000.0),

				_ => assert!(false)
			}
		}
	}
}

#[no_mangle]
pub extern fn onConnect(clientFD: ::std::libc::c_int, clientMap: &mut ::clients::ClientMap) {
	if (::clients::can_add(clientMap)) {
		let distance = 100.0;

		let alpha = 90.0 / 180.0 * ::std::f64::consts::PI;

		let pos = ::common::vec::Vec2 {
			x: distance * ::std::f64::cos(alpha),
			y: distance * ::std::f64::sin(alpha) };

		let vel = ::common::vec::Vec2 {
			x: 30.0,
			y: 0.0 };

		::clients::add(clientMap, clientFD, pos, vel);
	}
	else
	{
		unsafe {
			close(clientFD);
		}
	}
}

#[no_mangle]
pub extern fn onDisconnect(clientId: ::std::libc::size_t, clientMap: &mut ::clients::ClientMap, events: &mut Events) {
	::clients::remove(clientMap, clientId);

	unsafe {
		let mut i = 0;
		while i < clientMap.clients.cap {
			let client = (*::std::ptr::mut_offset(clientMap.clients.elems, i as int)).value;
			let status = ::protocol::send_remove(
				client.socketFD,
				clientId);

			if (status < 0) {
				let disconnectEvent = Event {
					theType: ON_DISCONNECT,
					onDisconnect: DisconnectEvent {
						clientId: i },
					onConnect: ConnectEvent { clientFD: 0 },
					onUpdate: UpdateEvent { dummy: 0 } };

				let ptr = ::std::ptr::mut_offset(events.buffer, (events.last % events.cap) as int);
				*ptr = disconnectEvent;
				events.last += 1;
			}

			i += 1;
		}
	}
}

#[no_mangle]
pub extern fn onUpdate(clientMap: &mut ::clients::ClientMap, events: &mut Events, dTimeInS: f64) {
	unsafe {
		let mut i = 0;
		while (i < clientMap.clients.cap) {
			if (*::std::ptr::mut_offset(clientMap.clients.elems, i as int)).isOccupied == 1 {
				let client = &mut (*::std::ptr::mut_offset(clientMap.clients.elems, i as int)).value;
				let ship = &mut client.ship;

				let gMag = 3000.0 / ship.pos.magnitude();
				let g = ship.pos.normalize() * -gMag;

				ship.pos = ship.pos + ship.vel * dTimeInS;
				ship.vel = ship.vel + g * dTimeInS;
			}

			i += 1;
		}

		i = 0;
		while (i < clientMap.clients.cap) {
			if (*::std::ptr::mut_offset(clientMap.clients.elems, i as int)).isOccupied == 1 {
				let mut j = 0;
				while (j < clientMap.clients.cap) {
					if (*::std::ptr::mut_offset(clientMap.clients.elems, j as int)).isOccupied == 1 {
						let status = ::protocol::send_update(
							(*::std::ptr::mut_offset(clientMap.clients.elems, i as int)).value.socketFD,
							(*::std::ptr::mut_offset(clientMap.clients.elems, j as int)).value.id,
							(*::std::ptr::mut_offset(clientMap.clients.elems, j as int)).value.ship.pos.x,
							(*::std::ptr::mut_offset(clientMap.clients.elems, j as int)).value.ship.pos.y);

						if (status < 0) {
							let disconnectEvent = Event {
								theType: ON_DISCONNECT,
								onDisconnect: DisconnectEvent {
									clientId: i },
								onConnect: ConnectEvent { clientFD: 0 },
								onUpdate: UpdateEvent { dummy: 0 } };

							let ptr = ::std::ptr::mut_offset(events.buffer, (events.last % events.cap) as int);
							*ptr = disconnectEvent;
							events.last += 1;
						}
					}

					j += 1;
				}
			}

			i += 1;
		}
	}
}
