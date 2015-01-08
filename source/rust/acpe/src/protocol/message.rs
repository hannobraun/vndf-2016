use std::default::Default;
use std::slice::Iter;
use std::vec::Drain;

use super::{
	decode,
	Decode,
	Encode,
	Encoder,
};


#[derive(Clone, PartialEq, Show)]
pub struct Message<Header, Id, Entity> {
	pub header: Header,

	// TODO: Control flow data

	// Main payload
	update : Vec<(Id, Entity)>,
	destroy: Vec<Id>,

	// TODO: Additional payload (generic)
}

impl<Header, Id, Entity> Message<Header, Id, Entity>
	where
		Header: Decode + Encode + Default,
		Id    : Decode + Encode,
		Entity: Decode + Encode,
{
	pub fn decode(
		buffer: &[u8]
	) -> Result<Message<Header, Id, Entity>, String> {
		// Performance note:
		// These allocations can be avoided by reusing an existing Message. In
		// that scheme, Message::new() would create an empty message, then
		// message.decode would decode a buffer, saving the result in that
		// message.
		let mut message = Message::new(Default::default());

		match decode(buffer, &mut message) {
			Ok(())     => Ok(message),
			Err(error) => Err(error),
		}
	}

	/// This is a convenience method that makes encoding as easy as possible,
	/// ignoring performance and error handling. Please don't use this outside
	/// of test code.
	pub fn encode(self) -> Vec<u8> {
		let mut encoder = Encoder::new();

		let mut message = encoder.message(&self.header);
		for (id, entity) in self.update.into_iter() {
			message.update(&id, &entity);
		}

		message.encode().to_vec()
	}
}

impl<Header, Id, Entity> Message<Header, Id, Entity> {
	pub fn new(header: Header) -> Message<Header, Id, Entity> {
		Message {
			header : header,
			update : Vec::new(),
			destroy: Vec::new(),
		}
	}

	pub fn update(&mut self, update: (Id, Entity)) {
		self.update.push(update);
	}

	pub fn destroy(&mut self, id: Id) {
		self.destroy.push(id);
	}

	pub fn update_items(&self) -> Iter<(Id, Entity)> {
		self.update.iter()
	}

	pub fn destroy_items(&self) -> Iter<Id> {
		self.destroy.iter()
	}

	pub fn drain_update_items(&mut self) -> Drain<(Id, Entity)> {
		self.update.drain()
	}

	pub fn drain_destroy_items(&mut self) -> Drain<Id> {
		self.destroy.drain()
	}
}


#[cfg(test)]
mod test {
	use super::Message;


	#[test]
	fn it_should_add_items() {
		let mut message = Message::new(0);

		let update  = (0, "This represents an entity.".to_string());
		let destroy = 1;

		message.update(update.clone());
		message.destroy(destroy);

		let updates : Vec<&(i32, String)> = message.update_items().collect();
		let destroys: Vec<&i32>           = message.destroy_items().collect();

		assert_eq!(vec![&update] , updates);
		assert_eq!(vec![&destroy], destroys);
	}

	#[test]
	fn it_should_provide_draining_iterators() {
		let mut message = Message::new(0);

		let update  = (0, "This represents an entity.".to_string());
		let destroy = 1;

		message.update(update.clone());
		message.destroy(destroy);

		let updates : Vec<(i32, String)> =
			message.drain_update_items().collect();
		let destroys: Vec<i32>           =
			message.drain_destroy_items().collect();

		assert_eq!(vec![update] , updates);
		assert_eq!(vec![destroy], destroys);
		assert_eq!(message.update_items().count() , 0);
		assert_eq!(message.destroy_items().count(), 0);
	}


}
