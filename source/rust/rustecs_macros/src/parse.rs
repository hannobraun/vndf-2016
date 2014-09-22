use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::parse;
use syntax::parse::common::seq_sep_trailing_disallowed;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::ptr::P;


pub fn parse(context: &ExtCtxt, token_tree: &[ast::TokenTree]) -> Vec<Entity> {
	let mut parser = parse::new_parser_from_tts(
		context.parse_sess(),
		context.cfg(),
		Vec::from_slice(token_tree)
	);

	let mut entities = Vec::new();

	loop {
		entities.push(Entity::parse(&mut parser));

		if parser.eat(&token::EOF) {
			break;
		}
	}

	entities
}


pub struct Entity {
	pub name      : ast::Ident,
	pub components: Vec<ast::Ident>,
	pub args      : Vec<ast::Arg>,
	pub init_block: P<ast::Block>,
}

impl Entity {
	fn parse(parser: &mut Parser) -> Entity {
		let name = parser.parse_ident();
		let components = parser.parse_unspanned_seq(
			&token::LPAREN,
			&token::RPAREN,
			seq_sep_trailing_disallowed(token::COMMA),
			|p| p.parse_ident());

		parser.expect(&token::COLON);

		let args = parser.parse_unspanned_seq(
			&token::BINOP(token::OR),
			&token::BINOP(token::OR),
			seq_sep_trailing_disallowed(token::COMMA),
			|p| p.parse_arg());

		let init_block = parser.parse_block();

		Entity {
			name      : name,
			components: components,
			args      : args,
			init_block: init_block
		}
	}
}
