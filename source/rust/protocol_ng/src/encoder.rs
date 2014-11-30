use super::{
	MAX_PACKET_SIZE,
	PerceptionEnc,
	Seq,
};


pub struct Encoder {
	buffer: [u8, ..MAX_PACKET_SIZE],
}

impl Encoder {
	pub fn new() -> Encoder {
		Encoder {
			buffer: [0, ..MAX_PACKET_SIZE],
		}
	}

	pub fn perception(&mut self, last_action: Seq) -> PerceptionEnc {
		PerceptionEnc::new(&mut self.buffer, last_action)
	}
}
