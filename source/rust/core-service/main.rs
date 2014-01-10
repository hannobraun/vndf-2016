#[crate_type = "rlib"];
#[crate_type = "staticlib"];
#[crate_id = "core-service"];

extern mod common;
extern mod net;

pub mod clients;
pub mod events;
pub mod protocol;
