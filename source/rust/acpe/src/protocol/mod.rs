use rustc_serialize::{
	json,
	Decodable,
};


pub use self::action::{
	Action,
	ActionHeader,
};
pub use self::decode::decode;
pub use self::encode::{
	Encode,
	Encoder,
	MessageEncoder,
};
pub use self::message::Message;
pub use self::perception::{
	Perception,
	PerceptionHeader,
};


mod action;
mod decode;
mod encode;
mod message;
mod perception;


pub type Seq = u64;


pub trait Decode {
	fn decode(s: &str) -> Result<Self, String>;
}

impl<T> Decode for T where T: Decodable<json::Decoder, json::DecoderError> {
	fn decode(s: &str) -> Result<Self, String> {
		json::decode(s)
			.map_err(|error|
				format!("JSON decoding error: {}", error)
			)
	}
}
