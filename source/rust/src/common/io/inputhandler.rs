use common::io::Input;

pub trait InputHandler {
	fn input(&mut self) -> Input;
}
