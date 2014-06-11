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


mod ecs;
mod generate;
mod parse;


#[macro_registrar]
pub fn macro_registrar(register: |ast::Name, SyntaxExtension|) {
	let ecs_expander = box BasicMacroExpander {
		expander: ecs::expand,
		span    : None
	};

	register(token::intern("ecs"), NormalTT(ecs_expander, None));
}
