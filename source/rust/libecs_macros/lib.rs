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


struct EntityMacro {
	items: Vec<@ast::Item>
}

impl MacResult for EntityMacro {
	fn make_items(&self) -> Option<SmallVector<@ast::Item>> {
		Some(SmallVector::many(self.items.clone()))
	}
}

fn expand_entity_macro(
	context   : &mut ExtCtxt,
	_         : codemap::Span,
	token_tree: &[ast::TokenTree]) -> Box<MacResult> {

	// Parse
	let mut parser = parse::new_parser_from_tts(
		context.parse_sess(),
		context.cfg(),
		Vec::from_slice(token_tree));

	let entity = parser.parse_ident();

	let component_types = parser.parse_unspanned_seq(
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

	// Done parsing, here we generate snippets for our entity implementation
	let components_args_names: Vec<ast::Ident> = component_types
		.iter()
		.enumerate()
		.map(|(i, _)|
			ast::Ident::new(token::intern(
				"cs".to_str().append(i.to_str().as_slice()).as_slice())))
		.collect();

	let component_names: Vec<ast::Ident> = component_types
		.iter()
		.enumerate()
		.map(|(i, _)|
			ast::Ident::new(token::intern(
				"c".to_str().append(i.to_str().as_slice()).as_slice())))
		.collect();

	let mut components_args: Vec<ast::TokenTree> = Vec::new();
	for (i, ty) in component_types.iter().enumerate() {
		let arg_name = components_args_names.get(i);

		components_args.push_all(
			quote_tokens!(&*context,
				$arg_name: &mut Components<$ty>
			).as_slice());

		if i + 1 < component_types.len() {
			components_args.push_all(
				quote_tokens!(&*context, ,).as_slice());
		}
	}

	let mut components_tuple: Vec<ast::TokenTree> = Vec::new();
	for (i, name) in component_names.iter().enumerate() {
		components_tuple.push_all(
			quote_tokens!(&*context, $name).as_slice());

		if i + 1 < component_names.len() {
			components_tuple.push_all(
				quote_tokens!(&*context, ,).as_slice());
		}
	}

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
	let macro = EntityMacro {
		items: vec!(
			quote_item!(&*context,
				struct $entity;
			).unwrap(),
			quote_item!(&*context,
				impl $entity {
					pub fn create(id: EntityId, $arg_name: $arg_type, $components_args) {
						let ($components_tuple) = $init_block;
						$components_inserts
					}
				}
			).unwrap()
		)
	};

	box macro as Box<MacResult>
}
