use game_service::ReceiveResult;


pub struct Receiver {
	pub received: Vec<ReceiveResult>,
}

impl Receiver {
	pub fn new() -> Receiver {
		Receiver {
			received: Vec::new(),
		}
	}
}