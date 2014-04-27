use collections::HashMap;
use libc::c_int;

use common::physics::Body;
use common::net::Connection;


pub type Id = uint;

pub struct Clients {
	map: HashMap<Id, Client>
}

impl Clients {
	pub fn new() -> Clients {
		Clients {
			map: HashMap::<Id, Client>::new()
		}
	}

	pub fn add<'a>(&'a mut self, client: Client) -> (Id, &'a Client) {
		let client_id = client.conn.fd as Id;
		self.map.insert(client_id, client);

		(
			client_id,
			self.map.get(&client_id))
	}

	pub fn client_by_fd<'a>(&'a mut self, fd: c_int) -> Option<(Id, &'a mut Client)> {
		let client_id = fd as Id;
		match self.map.find_mut(&client_id) {
			Some(client) => Some((client_id, client)),
			None         => None
		}
	}

	pub fn remove(&mut self, id: Id) -> Option<Client> {
		self.map.pop(&id)
	}

	pub fn each(&self, f: |Id, &Client|) {
		for (&id, client) in self.map.iter() {
			f(id, client);
		}
	}

	pub fn mut_each(&mut self, f: |Id, &mut Client|) {
		for (&id, client) in self.map.mut_iter() {
			f(id, client);
		}
	}
}


pub struct Client {
	pub conn   : Connection,
	pub ship   : Body,
	pub created: bool
}

impl Client {
	pub fn new(conn: Connection, ship: Body) -> Client {
		Client {
			conn   : conn,
			ship   : ship,
			created: false
		}
	}
}
