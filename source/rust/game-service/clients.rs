use collections::HashMap;
use libc::c_int;

use common::net::Connection;


pub type Id = uint;

pub struct Clients {
	pub map: HashMap<Id, Connection>
}

impl Clients {
	pub fn new() -> Clients {
		Clients {
			map: HashMap::<Id, Connection>::new()
		}
	}

	pub fn add<'a>(&'a mut self, conn: Connection) -> (Id, &'a Connection) {
		let client_id = conn.fd as Id;
		self.map.insert(client_id, conn);

		(
			client_id,
			self.map.get(&client_id))
	}

	pub fn client_by_fd<'a>(&'a mut self, fd: c_int) -> Option<(Id, &'a mut Connection)> {
		let client_id = fd as Id;
		match self.map.find_mut(&client_id) {
			Some(client) => Some((client_id, client)),
			None         => None
		}
	}

	pub fn remove(&mut self, id: Id) -> Option<Connection> {
		self.map.pop(&id)
	}
}
