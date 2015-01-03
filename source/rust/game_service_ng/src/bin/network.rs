use acpe::protocol::Seq;


pub struct Client {
	pub id           : String,
	pub last_action  : Seq,
	pub last_active_s: f64,
	pub broadcast    : Option<String>,
}
