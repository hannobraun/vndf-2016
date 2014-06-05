#![crate_id   = "ecs_macros"]
#![crate_type = "dylib"]
#![feature(macro_registrar, managed_boxes, quote)]


extern crate syntax;


use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{
	BasicMacroExpander,
	ExtCtxt,
	MacResult,
	NormalTT,
	SyntaxExtension
};
use syntax::parse;
use syntax::parse::common::seq_sep_trailing_disallowed;
use syntax::parse::token;
use syntax::util::small_vector::SmallVector;


#[macro_registrar]
pub fn macro_registrar(register: |ast::Name, SyntaxExtension|) {
	let expander = box BasicMacroExpander {
		expander: expand_entity_macro,
		span    : None
	};
	register(token::intern("entity"), NormalTT(expander, None))
}


struct EntityMacroResult {
	items: Vec<@ast::Item>
}

impl MacResult for EntityMacroResult {
	fn make_items(&self) -> Option<SmallVector<@ast::Item>> {
		Some(SmallVector::many(self.items.clone()))
	}
}

fn expand_entity_macro(
	context   : &mut ExtCtxt,
	_         : codemap::Span,
	token_tree: &[ast::TokenTree]) -> Box<MacResult> {

	let EntityMacro(entity, components, arg_name, arg_type, init_block) =
		EntityMacro::parse(context, token_tree);

	// Done parsing, here we generate snippets for our entity implementation
	let components_args_names = generate_components_args_names(&components);
	let component_names       = generate_component_names(&components);

	let components_args = generate_components_args(
		context,
		&components,
		&components_args_names);

	let component_tuple = generate_component_tuple(
		context,
		&component_names);

	let mut components_inserts: Vec<ast::TokenTree> = Vec::new();
	for (i, arg_name) in components_args_names.iter().enumerate() {
		let component_name = component_names.get(i);

		components_inserts.push_all(
			quote_tokens!(&*context,
				$arg_name.insert(id, $component_name);
			).as_slice());
	}

	// Done generating snippets. Now the snippets are put together into the
	// entity implementation.
	let macro = EntityMacroResult {
		items: vec!(
			quote_item!(&*context,
				struct $entity;
			).unwrap(),
			quote_item!(&*context,
				impl $entity {
					pub fn create(id: EntityId, $arg_name: $arg_type, $components_args) {
						let ($component_tuple) = $init_block;
						$components_inserts
					}
				}
			).unwrap()
		)
	};

	box macro as Box<MacResult>
}


struct EntityMacro(
	ast::Ident,
	Vec<@ast::Ty>,
	ast::Ident,
	@ast::Ty,
	@ast::Block,
);

impl EntityMacro {
	fn parse(context: &ExtCtxt, token_tree: &[ast::TokenTree]) -> EntityMacro {
		let mut parser = parse::new_parser_from_tts(
			context.parse_sess(),
			context.cfg(),
			Vec::from_slice(token_tree));

		let entity = parser.parse_ident();

		let components = parser.parse_unspanned_seq(
			&token::LT,
			&token::GT,
			seq_sep_trailing_disallowed(token::COMMA),
			|p| p.parse_ty(true));

		parser.expect(&token::COMMA);
		parser.expect(&token::BINOP(token::OR));
		let arg_name = parser.parse_ident();
		parser.expect(&token::COLON);
		let arg_type = parser.parse_ty(true);
		parser.expect(&token::BINOP(token::OR));
		let init_block = parser.parse_block();

		EntityMacro(
			entity,
			components,
			arg_name,
			arg_type,
			init_block
		)
	}
}


fn generate_components_args_names(components: &Vec<@ast::Ty>) -> Vec<ast::Ident> {
	components
		.iter()
		.enumerate()
		.map(|(i, _)|
			ast::Ident::new(token::intern(
				"cs".to_str().append(i.to_str().as_slice()).as_slice())))
		.collect()
}

fn generate_component_names(components: &Vec<@ast::Ty>) -> Vec<ast::Ident> {
	components
		.iter()
		.enumerate()
		.map(|(i, _)|
			ast::Ident::new(token::intern(
				"c".to_str().append(i.to_str().as_slice()).as_slice())))
		.collect()
}

fn generate_components_args(
	context   : &ExtCtxt,
	components: &Vec<@ast::Ty>,
	names     : &Vec<ast::Ident>) -> Vec<ast::TokenTree> {

	let mut components_args = Vec::new();

	for (i, ty) in components.iter().enumerate() {
		let arg_name = names.get(i);

		components_args.push_all(
			quote_tokens!(&*context,
				$arg_name: &mut Components<$ty>
			).as_slice());

		if i + 1 < components.len() {
			components_args.push_all(
				quote_tokens!(&*context, ,).as_slice());
		}
	}

	components_args
}

fn generate_component_tuple(
	context: &ExtCtxt,
	names  : &Vec<ast::Ident>) -> Vec<ast::TokenTree> {

	let mut component_tuple = Vec::new();

	for (i, name) in names.iter().enumerate() {
		component_tuple.push_all(
			quote_tokens!(&*context, $name).as_slice());

		if i + 1 < names.len() {
			component_tuple.push_all(
				quote_tokens!(&*context, ,).as_slice());
		}
	}

	component_tuple
}
