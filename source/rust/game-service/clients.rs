use collections::HashMap;

use common::physics::Body;
use common::net::Connection;


pub struct Clients {
	pub map   : ~HashMap<uint, Client>
}

impl Clients {
	pub fn new() -> ~Clients {
		~Clients {
			map: ~HashMap::<uint, Client>::new()
		}
	}

	/**
	 * Adds a client.
	 *
	 * The return value is a bit complicated. The reason for this is that
	 * calling this function will move the client into it, making it unusable
	 * for the caller.
	 * If the adding was successful, we borrow it back to the caller, so they
	 * can do whatever they need.
	 * If the adding was unsuccessful, we have no need for the client and move
	 * it back to the caller.
	 */
	pub fn add<'a>(&'a mut self, client: Client) -> Result<(uint, &'a Client), Client> {
		let client_id = client.conn.fd as uint;
		self.map.insert(client_id, client);

		Ok((
			client_id,
			self.map.get(&client_id)))
	}

	pub fn remove(&mut self, id: uint) {
		self.map.remove(&id);
	}

	pub fn each(&self, f: |uint, &Client|) {
		for (&id, client) in self.map.iter() {
			f(id, client);
		}
	}

	pub fn mut_each(&mut self, f: |uint, &mut Client|) {
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
