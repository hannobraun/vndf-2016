#[crate_type = "rlib"];
#[crate_type = "staticlib"];
#[crate_id = "core-service"];

extern mod clients;
extern mod common;
extern mod protocol;

pub mod events;
