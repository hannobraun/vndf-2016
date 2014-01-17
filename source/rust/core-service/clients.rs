use std::libc;
use std::ptr;

use common::dynamics;
use common::vec;


pub struct ClientMap {
	clients: IdMap,
	idPool : ~IdPool
}

pub struct IdMap {
	cap  : libc::size_t,
	elems: *mut IdMapEntry
}

pub struct IdMapEntry {
	isOccupied: int,
	value     : Client
}

struct Client {
	socketFD: libc::c_int,
	id      : libc::size_t,
	ship    : dynamics::Body
}

impl IdMap {
	fn init(&mut self, capacity: uint) {
		self.cap = capacity as u64;
		let memSize = capacity * ::std::mem::size_of::<IdMapEntry>();
		self.elems = unsafe { libc::malloc(memSize as u64) as *mut IdMapEntry };
		unsafe { ptr::set_memory(self.elems, 0, capacity as uint) };
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
					let mut client = (*ptr::mut_offset(self.elems, i as int)).value;
					f(&mut client);
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


pub fn new_client_map(cap: uint) -> ~ClientMap {
	let mut c = ~ClientMap {
		clients: IdMap {
			cap  : 0,
			elems: ::std::ptr::null::<IdMapEntry>() as *mut IdMapEntry },
		idPool: IdPool::new(cap as uint) };

	c.clients.init(cap);

	c
}

pub fn can_add(c: &ClientMap) -> bool {
	c.idPool.has_ids()
}

pub fn add(c: &mut ClientMap, socketFD: libc::c_int, pos: vec::Vec2, vel: vec::Vec2) {
	let clientId = c.idPool.pop();

	let client = Client {
		socketFD: socketFD,
		id      : clientId as u64,
		ship    : dynamics::Body { pos: pos, vel: vel } };

	c.clients.add(client);
}

pub fn remove(c: &mut ClientMap, id: uint) {
	if c.clients.remove(id) {
		c.idPool.push(id as uint);
	}
}
