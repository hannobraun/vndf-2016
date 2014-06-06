#![crate_id   = "rustecs_macros"]
#![crate_type = "dylib"]
#![feature(macro_registrar, managed_boxes, quote)]


extern crate syntax;


use syntax::ast;
use syntax::ext::base::{
	BasicMacroExpander,
	NormalTT,
	SyntaxExtension
};
use syntax::parse::token;


mod entity;


#[macro_registrar]
pub fn macro_registrar(register: |ast::Name, SyntaxExtension|) {
	let expander = box BasicMacroExpander {
		expander: entity::expand_macro,
		span    : None
	};
	register(token::intern("entity"), NormalTT(expander, None))
}
