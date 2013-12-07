#[crate_type = "rlib"];
#[link(name = "clients", package_id = "clients", vers = "0.0")];


extern mod vec;
extern mod dynamics;


struct ClientMap {
	clients: IdMap,
	idPool : Stack
}

struct IdMap {
	cap  : std::libc::size_t,
	elems: *mut IdMapEntry
}

struct IdMapEntry {
	isOccupied: int,
	value     : Client
}

struct Stack {
	cap  : std::libc::size_t,
	size : std::libc::size_t,
	elems: *mut std::libc::size_t
}

struct Client {
	socketFD: int,
	id:       std::libc::size_t,
	ship:     dynamics::Body
}


#[no_mangle]
pub extern fn clients_initClientMap(c: &mut ClientMap, cap: std::libc::size_t) {
	// Init IdMap
	c.clients.cap = cap;
	let memSize = cap * std::mem::size_of::<IdMapEntry>() as std::libc::size_t;
	c.clients.elems = unsafe { std::libc::malloc(memSize) as *mut IdMapEntry };
	unsafe { std::ptr::set_memory(c.clients.elems, 0, cap as uint) };

	// Init Stack
	c.idPool.cap = cap;
	c.idPool.size = cap;
	let idPoolSize =
		cap * std::mem::size_of::<std::libc::size_t>() as std::libc::size_t;
	c.idPool.elems = unsafe {
		std::libc::malloc(idPoolSize) as *mut std::libc::size_t };

	// Init ids
	let mut i: int = 0;
	while i < cap as int {
		unsafe {
			let ptr = std::ptr::mut_offset(c.idPool.elems, i);
			*ptr = (cap as int - i) as std::libc::size_t; };
		i += 1;
	}
}
