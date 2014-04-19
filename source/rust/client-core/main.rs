extern crate common;
extern crate libc;


use common::net::Connection;
use common::protocol::{Create, Message, Remove, SelfInfo, Update};


mod args;


fn main() {
	let (address, port) = args::address_and_port();

	let mut connection = Connection::connect(address, port);

	loop {
		connection.receive_messages(|message| {
			match Message::from_str(message) {
				SelfInfo(self_info) =>
					print!("{}\n", SelfInfo(self_info).to_str()),
				Create(create) =>
					print!("{}\n", Create(create).to_str()),
				Update(update) =>
					print!("{}\n", Update(update).to_str()),
				Remove(remove) =>
					print!("{}\n", Remove(remove).to_str()),

				_ =>
					fail!("invalid message ({})\n", message)
			}
		});
	}
}
