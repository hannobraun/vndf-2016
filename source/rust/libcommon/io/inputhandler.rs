use io::Input;

pub trait InputHandler {
	fn apply(&mut self) -> Input;
}
