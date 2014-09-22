use std::ascii::StrAsciiExt;
use syntax::ast;
use syntax::parse::token;


pub fn ident_to_lower(ident: ast::Ident) -> String {
	let camel_case = token::get_ident(ident).to_string().into_ascii();

	let mut snake_case = String::new();
	for (i, c) in camel_case.iter().enumerate() {
		if c.is_upper() && i != 0 {
			snake_case = snake_case.append("_");
		}

		snake_case.push_char(c.to_lower().to_char());
	}

	snake_case
}
