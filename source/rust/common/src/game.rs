#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq, Show)]
pub struct Broadcast {
	pub sender : String,
	pub message: String,
}
