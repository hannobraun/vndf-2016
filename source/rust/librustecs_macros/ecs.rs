use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{
	ExtCtxt,
	MacResult,
};
use syntax::util::small_vector::SmallVector;

use generate;
use parse;


pub fn expand(
	context   : &mut ExtCtxt,
	_         : codemap::Span,
	token_tree: &[ast::TokenTree]
	) -> Box<MacResult> {

	let parsed_ecs    = parse::ECS::parse(context, token_tree);
	let generated_ecs = generate::ECS::generate(context, &parsed_ecs);

	let result = MacroResult {
		items: vec!(
			generated_ecs.worlds.get(0).structure,
			generated_ecs.worlds.get(0).implementation
		)
	};

	box result as Box<MacResult>
}


struct MacroResult {
	items: Vec<@ast::Item>
}

impl MacResult for MacroResult {
	fn make_items(&self) -> Option<SmallVector<@ast::Item>> {
		Some(SmallVector::many(self.items.clone()))
	}
}
