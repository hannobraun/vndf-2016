use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::parse;
use syntax::parse::common::seq_sep_trailing_disallowed;
use syntax::parse::parser::Parser;
use syntax::parse::token;


pub struct ECS {
	pub components: Vec<Component>,
	pub entities  : Vec<Entity>,
	pub worlds    : Vec<World>,
}

impl ECS {
	pub fn parse(context: &ExtCtxt, token_tree: &[ast::TokenTree]) -> ECS {
		let mut parser = parse::new_parser_from_tts(
			context.parse_sess(),
			context.cfg(),
			Vec::from_slice(token_tree));

		let mut components = Vec::new();
		let mut entities   = Vec::new();
		let mut worlds     = Vec::new();

		loop {
			match Directive::parse(&mut parser) {
				ComponentDirective(component) => components.push(component),
				EntityDirective(entity)       => entities.push(entity),
				WorldDirective(world)         => worlds.push(world),
			}

			if parser.eat(&token::EOF) {
				break;
			}
		}

		ECS {
			components: components,
			entities  : entities,
			worlds    : worlds,
		}
	}
}


enum Directive {
	ComponentDirective(Component),
	EntityDirective(Entity),
	WorldDirective(World),
}

impl Directive {
	fn parse(parser: &mut Parser) -> Directive {
		let ident = parser.parse_ident();

		match parser.id_to_interned_str(ident).get() {
			"component" => ComponentDirective(Component::parse(parser)),
			"entity"    => EntityDirective(Entity::parse(parser)),
			"world"     => WorldDirective(World::parse(parser)),

			ident @ _ =>
				parser.fatal(format!("Unexpected identifier: {}", ident).as_slice())
		}
	}
}


pub struct Component {
	name      : ast::Ident,
	collection: ast::Ident,
	ty        : @ast::Ty,
}

impl Component {
	fn parse(parser: &mut Parser) -> Component {
		parser.expect(&token::LPAREN);

		let name = parser.parse_ident();
		parser.expect(&token::COMMA);
		let collection = parser.parse_ident();

		parser.expect(&token::RPAREN);
		parser.expect(&token::COLON);

		let ty = parser.parse_ty(false);

		Component {
			name      : name,
			collection: collection,
			ty        : ty,
		}
	}
}


pub struct Entity {
	name      : ast::Ident,
	components: Vec<ast::Ident>,
	args      : Vec<ast::Arg>,
	init_block: @ast::Block,
}

impl Entity {
	fn parse(parser: &mut Parser) -> Entity {
		parser.expect(&token::LPAREN);

		let name = parser.parse_ident();
		let components = parser.parse_unspanned_seq(
			&token::LT,
			&token::GT,
			seq_sep_trailing_disallowed(token::COMMA),
			|p| p.parse_ident());

		parser.expect(&token::RPAREN);
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


pub struct World {
	name    : ast::Ident,
	entities: Vec<ast::Ident>,
}

impl World {
	fn parse(parser: &mut Parser) -> World {
		parser.expect(&token::LPAREN);

		let name = parser.parse_ident();
		let entities = parser.parse_unspanned_seq(
			&token::LT,
			&token::GT,
			seq_sep_trailing_disallowed(token::COMMA),
			|p| p.parse_ident());

		parser.expect(&token::RPAREN);

		World {
			name    : name,
			entities: entities,
		}
	}
}
