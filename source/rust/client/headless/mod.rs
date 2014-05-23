use common::io;

pub use self::inputhandler::InputHandler;
pub use self::renderer::Renderer;


mod inputhandler;
mod renderer;


pub fn init() -> (Box<io::InputHandler>, Box<io::Renderer>) {
	(
		box InputHandler::new() as Box<io::InputHandler>,
		box Renderer::new() as Box<io::Renderer>)
}
