use syntax::ast;
use syntax::ext::base::ExtCtxt;

use parse;


pub struct World {
	pub structure     : @ast::Item,
	pub implementation: @ast::Item,
}

impl World {
	pub fn generate(context: &ExtCtxt, world: &parse::World) -> World {
		let name = world.name;

		let structure = quote_item!(&*context,
			pub struct $name {
				positions: ::rustecs::Components<Position>,
				visuals  : ::rustecs::Components<Visual>,
				scores   : ::rustecs::Components<u32>,
			}
		);

		let implementation = quote_item!(&*context,
			impl $name {
				pub fn new() -> $name {
					$name {
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
