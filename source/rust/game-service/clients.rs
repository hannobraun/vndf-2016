use collections::HashMap;

use common::physics::Body;
use common::net::Connection;


pub struct Clients {
	pub map   : ~HashMap<uint, Client>,
	pub idPool: ~IdPool
}

impl Clients {
	pub fn new(capacity: uint) -> ~Clients {
		~Clients {
			map   : ~HashMap::<uint, Client>::new(),
			idPool: IdPool::new(capacity) }
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
		if self.idPool.has_ids() {
			let client_id = self.idPool.pop();
			self.map.insert(client_id, client);

			Ok((
				client_id,
				self.map.get(&client_id)))
		}
		else {
			Err(client)
		}
	}

	pub fn remove(&mut self, id: uint) {
		if self.map.remove(&id) {
			self.idPool.push(id);
		}
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
	pub conn: Connection,
	pub ship: Body
}

impl Client {
	pub fn new(conn: Connection, ship: Body) -> Client {
		Client {
			conn: conn,
			ship: ship
		}
	}
}


pub struct IdPool {
	capacity: uint,
	pool    : Vec<uint>
}

impl IdPool {
	fn new(capacity: uint) -> ~IdPool {
		let mut idPool = ~IdPool {
			capacity: capacity,
			pool    : Vec::new() };

		let mut i = 0;
		while i < capacity {
			idPool.pool.push(capacity - i - 1);
			i += 1
		}

		idPool
	}

	fn has_ids(&self) -> bool {
		self.pool.len() > 0
	}

	fn push(&mut self, id: uint) {
		self.pool.push(id);
	}

	fn pop(&mut self) -> uint {
		match self.pool.pop() {
			Some(id) => id,
			None     => fail!("No id available.")
		}
	}
}
