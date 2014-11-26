extern crate serialize;


use serialize::json;


#[deriving(Encodable)]
pub enum Message {
	Login,
	Broadcast(String),
}

impl Message {
	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}
