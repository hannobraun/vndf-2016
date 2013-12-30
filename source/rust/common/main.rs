#[crate_type = "rlib"];
#[crate_type = "staticlib"];
#[link(name = "common", package_id = "common", vers = "0.0")];

pub mod dynamics;
pub mod vec;
