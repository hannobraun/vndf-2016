#[crate_id = "client"];
#[crate_type = "staticlib"];

extern mod common;

extern mod gl;
extern mod glfw;
extern mod stb_image;

pub mod camera;
pub mod display;
pub mod images;
pub mod input;
pub mod net;
pub mod texture;

#[link(name = "stb-image", kind = "static")]
extern {}
