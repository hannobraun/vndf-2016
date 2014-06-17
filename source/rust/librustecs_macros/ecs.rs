use std::gc::Gc;
use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{
	ExtCtxt,
	MacResult,
};
use syntax::util::small_vector::SmallVector;

use generate;
use parse::ECS;


pub fn expand(
	context   : &mut ExtCtxt,
	_         : codemap::Span,
	token_tree: &[ast::TokenTree]
	) -> Box<MacResult> {

	let ecs = ECS::parse(context, token_tree);

	let items = generate::items(context, &ecs);

	let result = MacroResult {
		items: items
	};

	box result as Box<MacResult>
}


struct MacroResult {
	items: Vec<Gc<ast::Item>>
}

impl MacResult for MacroResult {
	fn make_items(&self) -> Option<SmallVector<Gc<ast::Item>>> {
		Some(SmallVector::many(self.items.clone()))
	}
}
