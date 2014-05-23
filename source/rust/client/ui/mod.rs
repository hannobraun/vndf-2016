use std::rc::Rc;

use common::io;


pub use ui::font::Font;
pub use ui::inputhandler::InputHandler;
pub use ui::renderer::Renderer;
pub use ui::shaders::Shaders;
pub use ui::textures::{Texture, Textures};
pub use ui::window::Window;


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
