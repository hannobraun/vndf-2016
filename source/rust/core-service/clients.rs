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

pub struct IdPool {
	capacity: uint,
	pool    : ~[uint]
}

struct Client {
	socketFD: libc::c_int,
	id      : libc::size_t,
	ship    : dynamics::Body
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


pub fn new_client_map(cap: libc::size_t) -> ~ClientMap {
	let mut c = ~ClientMap {
		clients: IdMap {
			cap  : 0,
			elems: ::std::ptr::null::<IdMapEntry>() as *mut IdMapEntry },
		idPool: IdPool::new(cap as uint) };

	// Init IdMap
	c.clients.cap = cap;
	let memSize = cap * ::std::mem::size_of::<IdMapEntry>() as libc::size_t;
	c.clients.elems = unsafe { libc::malloc(memSize) as *mut IdMapEntry };
	unsafe { ptr::set_memory(c.clients.elems, 0, cap as uint) };

	c
}

pub fn can_add(c: &ClientMap) -> bool {
	c.idPool.has_ids()
}

pub fn add(c: &mut ClientMap, socketFD: libc::c_int, pos: vec::Vec2, vel: vec::Vec2) {
	// Get id from pool.
	let clientId = c.idPool.pop();

	// Construct client
	let client = Client {
		socketFD: socketFD,
		id      : clientId as u64,
		ship    : dynamics::Body { pos: pos, vel: vel } };

	// Add client to map
	unsafe {
		let ptr = ptr::mut_offset(c.clients.elems, clientId as int);
		(*ptr).isOccupied = 1;
		(*ptr).value = client;
	};
}

pub fn remove(c: &mut ClientMap, id: libc::size_t) {
	unsafe {
		let clientPtr = ptr::mut_offset(c.clients.elems, id as int);
		let containsClient = (*clientPtr).isOccupied == 1;

		if containsClient {
			// Remove client
			(*clientPtr).isOccupied = 0;

			c.idPool.push(id as uint);
		}
	}
}
