pub struct ClientMap {
	clients: IdMap,
	idPool : Stack
}

struct IdMap {
	cap  : ::std::libc::size_t,
	elems: *mut IdMapEntry
}

struct IdMapEntry {
	isOccupied: int,
	value     : Client
}

struct Stack {
	cap  : ::std::libc::size_t,
	size : ::std::libc::size_t,
	elems: *mut ::std::libc::size_t
}

struct Client {
	socketFD: ::std::libc::c_int,
	id:       ::std::libc::size_t,
	ship:     ::common::dynamics::Body
}


#[no_mangle]
pub extern fn clients_initClientMap(c: &mut ClientMap, cap: ::std::libc::size_t) {
	// Init IdMap
	c.clients.cap = cap;
	let memSize = cap * ::std::mem::size_of::<IdMapEntry>() as ::std::libc::size_t;
	c.clients.elems = unsafe { ::std::libc::malloc(memSize) as *mut IdMapEntry };
	unsafe { ::std::ptr::set_memory(c.clients.elems, 0, cap as uint) };

	// Init Stack
	c.idPool.cap = cap;
	c.idPool.size = cap;
	let idPoolSize =
		cap * ::std::mem::size_of::<::std::libc::size_t>() as ::std::libc::size_t;
	c.idPool.elems = unsafe {
		::std::libc::malloc(idPoolSize) as *mut ::std::libc::size_t };

	// Init ids
	let mut i: int = 0;
	while i < cap as int {
		unsafe {
			let ptr = ::std::ptr::mut_offset(c.idPool.elems, i);
			*ptr = (cap as int - i - 1) as ::std::libc::size_t; };
		i += 1;
	}
}

#[no_mangle]
pub extern fn clients_canAdd(c: &ClientMap) -> bool {
	c.idPool.size > 0
}

#[no_mangle]
pub extern fn clients_add(c: &mut ClientMap, socketFD: ::std::libc::c_int, pos: ::common::vec::Vec2, vel: ::common::vec::Vec2) {
	// Get id from pool.
	let clientId = unsafe {
		let ptr = ::std::ptr::mut_offset(c.idPool.elems, c.idPool.size as int);
		*ptr };
	c.idPool.size -= 1;

	// Construct client
	let client = Client {
		socketFD: socketFD,
		id      : clientId,
		ship    : ::common::dynamics::Body { pos: pos, vel: vel } };

	// Add client to map
	unsafe {
		let ptr = ::std::ptr::mut_offset(c.clients.elems, clientId as int);
		(*ptr).isOccupied = 1;
		(*ptr).value = client;
	};
}

#[no_mangle]
pub extern fn clients_remove(c: &mut ClientMap, id: ::std::libc::size_t) {
	unsafe {
		let clientPtr = ::std::ptr::mut_offset(c.clients.elems, id as int);
		let containsClient = (*clientPtr).isOccupied == 1;

		if containsClient {
			// Remove client
			(*clientPtr).isOccupied = 0;

			// Add id back to pool
			let idPtr =
				::std::ptr::mut_offset(c.idPool.elems, c.idPool.size as int);
			(*idPtr) = id;
			c.idPool.size += 1;
		}
	}
}
