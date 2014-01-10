#[crate_type = "rlib"];
#[crate_type = "staticlib"];
#[crate_id = "core-service"];

extern mod common;

pub mod clients;
pub mod events;
pub mod net;
pub mod protocol;
