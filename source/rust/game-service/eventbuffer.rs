use collections::Deque;
use collections::RingBuf;


pub struct Events<T> {
	buffer: RingBuf<T>
}

impl<T> Events<T> {
	pub fn new() -> Events<T> {
		Events {
			buffer: RingBuf::new()
		}
	}

	pub fn push(&mut self, event: T) {
		self.buffer.push_back(event)
	}

	pub fn pull(&mut self) -> Option<T> {
		self.buffer.pop_front()
	}
}
