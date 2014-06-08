use syntax::ast;
use syntax::ext::base::ExtCtxt;

use parse;


pub struct World {
	pub structure     : @ast::Item,
	pub implementation: @ast::Item,
}

impl World {
	pub fn generate(context: &ExtCtxt, _: &parse::World) -> World {
		let structure = quote_item!(&*context,
			pub struct World {
				positions: ::rustecs::Components<Position>,
				visuals  : ::rustecs::Components<Visual>,
				scores   : ::rustecs::Components<u32>,
			}
		);

		let implementation = quote_item!(&*context,
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

		World {
			structure     : structure.unwrap(),
			implementation: implementation.unwrap()
		}
	}
}
