use std::rc::Rc;

use common::io;

use self::inputhandler::InputHandler;
use self::renderer::Renderer;


pub use self::font::Font;
pub use self::shaders::Shaders;
pub use self::textures::{Texture, Textures};
pub use self::window::Window;


mod font;
mod images;
mod inputhandler;
mod renderer;
mod shaders;
mod textures;
mod window;


pub fn init() -> (Box<io::InputHandler>, Box<io::Renderer>) {
	let screen_width  = 800;
	let screen_height = 600;

	let     window   = Rc::new(Window::create(screen_width, screen_height));
	let     shaders  = Shaders::new(&*window);
	let mut textures = Textures::init(&*window);
	let     font     = Font::load(&mut textures);

	images::load(&mut textures);

	(
		box InputHandler::new(window.clone()) as Box<io::InputHandler>,
		box Renderer::new(
			window.clone(),
			shaders,
			textures,
			font) as Box<io::Renderer>)
}
