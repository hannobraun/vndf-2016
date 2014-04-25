use common::net::Acceptor;
use common::net::epoll;
use common::net::epoll::EPoll;


pub struct Network {
	pub epoll   : EPoll,
	pub acceptor: Acceptor
}

impl Network {
	pub fn new(port: ~str) -> Network {
		let epoll = match EPoll::create() {
			Ok(epoll)  => epoll,
			Err(error) => fail!("Error initializing epoll: {}", error)
		};

		let acceptor = Acceptor::create(port);

		match epoll.add(acceptor.fd, epoll::ffi::EPOLLIN) {
			Ok(()) => (),

			Err(error) =>
				fail!("Error registering server socket with epoll: {}", error)
		}

		Network {
			epoll   : epoll,
			acceptor: acceptor
		}
	}
}