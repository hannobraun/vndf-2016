#[derive(Clone, Eq, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub struct Broadcast {
	pub sender : String,
	pub message: String,
}
