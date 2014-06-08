use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{
	ExtCtxt,
	MacResult,
};
use syntax::util::small_vector::SmallVector;

use generate::World;
use parse::ECS;


pub fn expand(
	context   : &mut ExtCtxt,
	_         : codemap::Span,
	token_tree: &[ast::TokenTree]
	) -> Box<MacResult> {

	let ecs = ECS::parse(context, token_tree);

	let world = World::generate(context, ecs.worlds.get(0));

	let result = MacroResult {
		items: vec!(
			world.structure,
			world.implementation
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
