mod buffer;
mod color;
mod screen;
mod util;


pub use self::buffer::C;
pub use self::buffer::ScreenBuffer;
pub use self::color::Color;
pub use self::screen::Screen;
pub use self::util::Section;


pub type Pos = u16;
