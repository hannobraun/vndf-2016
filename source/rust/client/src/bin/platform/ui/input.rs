pub trait ProcessInput<T> {
	fn process_char(&mut self, element: &mut T, c: char);
}
