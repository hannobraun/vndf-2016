use common::net::epoll::EPoll;


pub struct Network {
	pub epoll: EPoll
}

impl Network {
	pub fn new() -> Network {
		let epoll = match EPoll::create() {
			Ok(epoll)  => epoll,
			Err(error) => fail!("Error initializing epoll: {}", error)
		};

		Network {
			epoll: epoll
		}
	}
}