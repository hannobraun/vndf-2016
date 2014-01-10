#[crate_type = "rlib"];
#[crate_type = "staticlib"];
#[crate_id = "core-service"];

extern mod common;
extern mod protocol;

pub mod clients;
pub mod events;
