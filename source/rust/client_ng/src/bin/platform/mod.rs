use self::input::ReadInput;
use self::render::Render;


pub mod input;
pub mod render;


// TODO: Merge ReadInput and Render into PlatformIo
pub trait PlatformIo: ReadInput + Render {}
