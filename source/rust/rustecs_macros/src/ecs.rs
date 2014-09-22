use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{
	ExtCtxt,
	MacResult,
};
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;

use generate;
use parse::parse;


pub fn expand(
	context   : &mut ExtCtxt,
	_         : codemap::Span,
	token_tree: &[ast::TokenTree]
) -> Box<MacResult + 'static> {
	let ecs = parse(context, token_tree);

	let items = generate::items(context, &ecs);

	let result = MacroResult {
		items: items
	};

	box result as Box<MacResult>
}


struct MacroResult {
	items: Vec<P<ast::Item>>
}

impl MacResult for MacroResult {
	fn make_items(self: Box<MacroResult>) -> Option<SmallVector<P<ast::Item>>> {
		Some(SmallVector::many(self.items.clone()))
	}
}
