#[crate_type = "rlib"];
#[crate_type = "staticlib"];
#[link(name = "protocol", package_id = "protocol", vers = "0.0")];


extern mod clients;
extern mod protocol;
extern mod vec;


extern {
	fn close(fd: std::libc::c_int) -> std::libc::c_int;
}


static ON_CONNECT: std::libc::c_int    = 0;
static ON_DISCONNECT: std::libc::c_int = 1;
static ON_UPDATE: std::libc::c_int     = 2;

struct Event {
	theType: std::libc::c_int,

	onConnect   : ConnectEvent,
	onDisconnect: DisconnectEvent,
	onUpdate    : UpdateEvent
}

struct ConnectEvent {
	clientFD: std::libc::c_int
}

struct DisconnectEvent {
	clientId: std::libc::size_t
}

struct UpdateEvent {
	dummy: std::libc::c_int
}

struct Events {
	first : u64,
	last  : u64,
	cap   : std::libc::size_t,
	buffer: *mut Event
}


#[no_mangle]
pub extern fn onConnect(clientFD: std::libc::c_int, clientMap: &mut clients::ClientMap) {
	if (clients::clients_canAdd(clientMap)) {
		let distance = 100.0;

		let alpha = 90.0 / 180.0 * std::f64::consts::PI;

		let pos = vec::Vec2 {
			x: distance * std::f64::cos(alpha),
			y: distance * std::f64::sin(alpha) };

		let vel = vec::Vec2 {
			x: 30.0,
			y: 0.0 };

		clients::clients_add(clientMap, clientFD, pos, vel);
	}
	else
	{
		unsafe {
			close(clientFD);
		}
	}
}

#[no_mangle]
pub extern fn onDisconnect(clientId: std::libc::size_t, clientMap: &mut clients::ClientMap, events: &mut Events) {
	clients::clients_remove(clientMap, clientId);

	unsafe {
		let mut i = 0;
		while i < clientMap.clients.cap {
			let client = (*std::ptr::mut_offset(clientMap.clients.elems, i as int)).value;
			let status = protocol::sendRemove(
				client.socketFD,
				clientId);

			if (status < 0) {
				let disconnectEvent = Event {
					theType: ON_DISCONNECT,
					onDisconnect: DisconnectEvent {
						clientId: i },
					onConnect: ConnectEvent { clientFD: 0 },
					onUpdate: UpdateEvent { dummy: 0 } };

				let ptr = std::ptr::mut_offset(events.buffer, (events.last % events.cap) as int);
				*ptr = disconnectEvent;
				events.last += 1;
			}

			i += 1;
		}
	}
}
