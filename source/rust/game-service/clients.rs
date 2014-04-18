use collections::HashMap;

use common::physics::Body;
use common::net::Connection;


pub struct Clients {
	pub map   : ~HashMap<uint, Client>,
	pub idPool: ~IdPool
}

pub struct Client {
	pub conn: Connection,
	pub id  : uint,
	pub ship: Body
}

impl Clients {
	pub fn new(capacity: uint) -> ~Clients {
		~Clients {
			map   : ~HashMap::<uint, Client>::new(),
			idPool: IdPool::new(capacity) }
	}

	pub fn add(&mut self, conn: Connection, ship: Body) -> Option<Client> {
		if self.idPool.has_ids() {
			let client_id = self.idPool.pop();

			let client = Client {
				conn: conn,
				id  : client_id,
				ship: ship };

			self.map.insert(client.id, client);

			Some(client)
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

	pub fn each(&self, f: |&Client|) {
		for (_, client) in self.map.iter() {
			f(client);
		}
	}

	pub fn mut_each(&mut self, f: |&mut Client|) {
		for (_, client) in self.map.mut_iter() {
			f(client);
		}
	}
}


pub struct IdPool {
	capacity: uint,
	pool    : ~[uint]
}

impl IdPool {
	fn new(capacity: uint) -> ~IdPool {
		let mut idPool = ~IdPool {
			capacity: capacity,
			pool    : ~[] };

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
