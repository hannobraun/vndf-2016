#![crate_name = "rustecs_macros"]
#![crate_type = "dylib"]
#![feature(plugin_registrar, managed_boxes, quote)]


extern crate rustc;
extern crate syntax;


use rustc::plugin::registry::Registry;

mod ecs;
mod generate;
mod names;
mod parse;


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_macro("ecs", ecs::expand);
}
