use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{
	ExtCtxt,
	MacResult,
};
use syntax::util::small_vector::SmallVector;

use parse::ECS;


pub fn expand(
	context   : &mut ExtCtxt,
	_         : codemap::Span,
	token_tree: &[ast::TokenTree]
	) -> Box<MacResult> {

	let ecs = ECS::parse(context, token_tree);

	let world_struct = quote_item!(&*context,
		pub struct World {
			positions: ::rustecs::Components<Position>,
			visuals  : ::rustecs::Components<Visual>,
			scores   : ::rustecs::Components<u32>,
		}
	);

	let world_impl = quote_item!(&*context,
		impl World {
			pub fn new() -> World {
				World {
					positions: ::rustecs::components(),
					visuals  : ::rustecs::components(),
					scores   : ::rustecs::components(),
				}
			}
		}
	);

	let result = MacroResult {
		items: vec!(
			world_struct.unwrap(),
			world_impl.unwrap()
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
