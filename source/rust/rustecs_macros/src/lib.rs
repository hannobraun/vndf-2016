#![feature(plugin_registrar, quote, tuple_indexing)]


extern crate rustc;
extern crate syntax;


use rustc::plugin::registry::Registry;

mod ecs;
mod generate;
mod names;
mod parse;


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_macro("world", ecs::expand);
}
