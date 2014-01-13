use std::libc;
use std::ptr;

use common::dynamics;
use common::vec;


pub struct ClientMap {
	clients: IdMap,
	idPool : Stack
}

pub struct IdMap {
	cap  : libc::size_t,
	elems: *mut IdMapEntry
}

pub struct IdMapEntry {
	isOccupied: int,
	value     : Client
}

pub struct Stack {
	cap  : libc::size_t,
	size : libc::size_t,
	elems: *mut libc::size_t
}

struct Client {
	socketFD: libc::c_int,
	id      : libc::size_t,
	ship    : dynamics::Body
}

impl Stack {
	fn init(&mut self, capacity: libc::size_t) {
		let idPoolSize =
			capacity * ::std::mem::size_of::<libc::size_t>() as libc::size_t;

		self.cap = capacity;
		self.size = 0;
		self.elems = unsafe {
			libc::malloc(idPoolSize) as *mut libc::size_t };

		while self.size < capacity {
			unsafe {
				let ptr = ptr::mut_offset(self.elems, self.size as int);
				*ptr = (capacity - self.size - 1) as libc::size_t; };

			self.size += 1;
		}
	}
}


pub fn new_client_map(cap: libc::size_t) -> ~ClientMap {
	let mut c = ClientMap {
		clients: IdMap {
			cap  : 0,
			elems: ::std::ptr::null::<IdMapEntry>() as *mut IdMapEntry },
		idPool: Stack {
			cap  : 0,
			size : 0,
			elems: ::std::ptr::null::<u64>() as *mut u64 } };

	// Init IdMap
	c.clients.cap = cap;
	let memSize = cap * ::std::mem::size_of::<IdMapEntry>() as libc::size_t;
	c.clients.elems = unsafe { libc::malloc(memSize) as *mut IdMapEntry };
	unsafe { ptr::set_memory(c.clients.elems, 0, cap as uint) };

	c.idPool.init(cap);

	~c
}

pub fn can_add(c: &ClientMap) -> bool {
	c.idPool.size > 0
}

pub fn add(c: &mut ClientMap, socketFD: libc::c_int, pos: vec::Vec2, vel: vec::Vec2) {
	// Get id from pool.
	let clientId = unsafe {
		let ptr = ptr::mut_offset(c.idPool.elems, (c.idPool.size - 1) as int);
		*ptr };
	c.idPool.size -= 1;

	// Construct client
	let client = Client {
		socketFD: socketFD,
		id      : clientId,
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

			// Add id back to pool
			let idPtr =
				ptr::mut_offset(c.idPool.elems, c.idPool.size as int);
			(*idPtr) = id;
			c.idPool.size += 1;
		}
	}
}
