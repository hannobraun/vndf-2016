use acpe::protocol::Encoder;


pub struct Sender {
	pub encoder: Encoder,
}

impl Sender {
	pub fn new() -> Sender {
		Sender {
			encoder: Encoder::new(),
		}
	}
}
