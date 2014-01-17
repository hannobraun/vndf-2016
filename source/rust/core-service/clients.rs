use std::libc;
use std::ptr;

use common::dynamics;
use common::vec;


pub struct Clients {
	clients: IdMap,
	idPool : ~IdPool
}

impl Clients {
	pub fn new(capacity: uint) -> ~Clients {
		let mut c = ~Clients {
			clients: IdMap {
				cap  : 0,
				elems: ::std::ptr::null::<IdMapEntry>() as *mut IdMapEntry },
			idPool: IdPool::new(capacity) };

		c.clients.init(capacity);

		c
	}

	pub fn add(&mut self, socketFD: libc::c_int, pos: vec::Vec2, vel: vec::Vec2) -> bool {
		if self.idPool.has_ids() {
			let clientId = self.idPool.pop();

			let client = Client {
				socketFD: socketFD,
				id      : clientId,
				ship    : dynamics::Body { pos: pos, vel: vel } };

			self.clients.add(client);

			true
		}
		else {
			false
		}
	}

	pub fn remove(&mut self, id: uint) {
		if self.clients.remove(id) {
			self.idPool.push(id);
		}
	}

	pub fn each(&self, f: |&mut Client|) {
		self.clients.each(f);
	}
}

struct IdMap {
	cap  : libc::size_t,
	elems: *mut IdMapEntry
}

struct IdMapEntry {
	isOccupied: int,
	value     : Client
}

struct Client {
	socketFD: libc::c_int,
	id      : uint,
	ship    : dynamics::Body
}

impl IdMap {
	fn init(&mut self, capacity: uint) {
		self.cap = capacity as u64;
		let memSize = capacity * ::std::mem::size_of::<IdMapEntry>();
		self.elems = unsafe { libc::malloc(memSize as u64) as *mut IdMapEntry };
		unsafe { ptr::set_memory(self.elems, 0, capacity) };
	}

	fn add(&mut self, client: Client) {
		unsafe {
			let ptr = ptr::mut_offset(self.elems, client.id as int);
			(*ptr).isOccupied = 1;
			(*ptr).value = client;
		};
	}

	fn remove(&mut self, id: uint) -> bool {
		unsafe {
			let clientPtr = ptr::mut_offset(self.elems, id as int);
			let containsClient = (*clientPtr).isOccupied == 1;

			if containsClient {
				(*clientPtr).isOccupied = 0;
			}

			containsClient
		}
	}

	pub fn each(&self, f: |&mut Client|) {
		unsafe {
			let mut i = 0;
			while i < self.cap {
				if (*::std::ptr::mut_offset(self.elems, i as int)).isOccupied == 1 {
					f(&mut (*ptr::mut_offset(self.elems, i as int)).value);
				}

				i += 1;
			}
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
			idPool.pool.push(capacity - i);
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
		self.pool.pop()
	}
}
