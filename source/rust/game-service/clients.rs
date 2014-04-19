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

	pub fn add(&mut self, conn: Connection, ship: Body) -> Option<(uint, Client)> {
		if self.idPool.has_ids() {
			let client_id = self.idPool.pop();

			let client = Client::new(conn, ship);

			self.map.insert(client_id, client);

			Some((client_id, client))
		}
		else {
			None
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

	pub fn mut_each(&mut self, f: |&mut Client|) {
		for (_, client) in self.map.mut_iter() {
			f(client);
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
