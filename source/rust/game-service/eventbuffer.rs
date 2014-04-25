use collections::Deque;
use collections::RingBuf;


pub struct EventBuffer<T> {
	buffer: RingBuf<T>
}

impl<T> EventBuffer<T> {
	pub fn new() -> EventBuffer<T> {
		EventBuffer {
			buffer: RingBuf::new()
		}
	}

	pub fn push(&mut self, event: T) {
		self.buffer.push_back(event)
	}

	pub fn pop(&mut self) -> Option<T> {
		self.buffer.pop_front()
	}
}
