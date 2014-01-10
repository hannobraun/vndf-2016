#[crate_type = "rlib"];
#[crate_type = "staticlib"];
#[crate_id = "core-service"];

extern mod common;
extern mod extra;

pub mod clients;
pub mod events;
pub mod net;
pub mod protocol;
pub mod util;
