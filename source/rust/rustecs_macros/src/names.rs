use std::ascii::StrAsciiExt;
use syntax::ast;
use syntax::parse::token;


pub fn ident_to_lower(ident: ast::Ident) -> String {
	token::get_ident(ident)
		.to_string()
		.as_slice()
		.to_ascii_lower()
}
