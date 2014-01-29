#[crate_id = "client"];
#[crate_type = "staticlib"];

extern mod common;

extern mod glfw;
extern mod stb_image;

pub mod images;
pub mod input;
pub mod net;
