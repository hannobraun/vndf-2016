extern crate common;
extern crate libc;

use std::comm;
use std::io;
use std::task;

use common::net::Connection;
use common::protocol::{Create, Message, Remove, SelfInfo, Update};


mod args;


fn main() {
	let (address, port) = args::address_and_port();

	let (tx, rx) = comm::channel();
	task::spawn(proc() {
		let mut stdin = io::stdin();
		loop {
			match stdin.read_line() {
				Ok(message) => tx.send(message),
				Err(error)  => fail!("Error reading from stdin: {}", error)
			}
		}
	});

	let mut connection = match Connection::connect(address, port) {
		Ok(connection) => connection,
		Err(error)     => {
			print!("Error connecting to server: {}\n", error);
			unsafe { libc::exit(1) }
		}
	};

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

		match rx.try_recv() {
			Ok(message) => {
				match connection.send_message(message) {
					Ok(())     => (),
					Err(error) => fail!("Error sending message: {}", error)
				}
			},

			_  => ()
		};
	}
}
