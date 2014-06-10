use std::collections::HashMap;
use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;

use parse;


pub fn items(context: &ExtCtxt, ecs: &parse::ECS) -> Vec<@ast::Item> {
	let components: HashMap<String, Component> = ecs.components
		.iter()
		.map(|component| {
			let component = Component::generate(context, component);
			(component.name.clone(), component)
		})
		.collect();

	let entities: Vec<Entity> = ecs.entities
		.iter()
		.map(|entity|
			Entity::generate(context, entity, &components))
		.collect();

	let worlds: Vec<World> = ecs.worlds
		.iter()
		.map(|world|
			World::generate(context, world, &entities))
		.collect();

	let mut items = Vec::new();
	for world in worlds.iter() {
		items.push(world.structure);
		items.push(world.implementation);
	}

	items
}


#[deriving(Clone)]
pub struct Component {
	name: String,
	decl: Vec<ast::TokenTree>,
}

impl Component {
	fn generate(context: &ExtCtxt, component: &parse::Component) -> Component {
		let name = token::get_ident(component.name).to_str();

		let collection = component.collection;
		let ty         = component.ty;

		let decl = quote_tokens!(&*context,
			$collection: ::rustecs::Components<$ty>,
		);

		Component {
			name: name,
			decl: decl,
		}
	}
}


pub struct Entity {
	components: HashMap<String, Component>,
}

impl Entity {
	fn generate(
		_             : &ExtCtxt,
		entity        : &parse::Entity,
		all_components: &HashMap<String, Component>
	) -> Entity {
		let entity_components = entity.components
			.iter()
			.map(|&ident| {
				let name = token::get_ident(ident).to_str();
				(name.clone(), (*all_components.get(&name)).clone())
			})
			.collect();

		Entity {
			components: entity_components
		}
	}
}


pub struct World {
	pub structure     : @ast::Item,
	pub implementation: @ast::Item,
}

impl World {
	fn generate(
		context   : &ExtCtxt,
		world     : &parse::World,
		entities  : &Vec<Entity>
	) -> World {
		let components = World::components(entities);

		let name  = world.name;
		let decls = World::component_decls(context, &components);

		let structure = quote_item!(&*context,
			pub struct $name {
				$decls
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

	fn components(entities: &Vec<Entity>) -> HashMap<String, Component> {
		let mut components = HashMap::new();

		for entity in entities.iter() {
			for (name, component) in entity.components.iter() {
				components.insert((*name).clone(), (*component).clone());
			}
		}

		components
	}

	fn component_decls(
		context   : &ExtCtxt,
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut tokens = vec!();

		for (_, component) in components.iter() {
			let decl = &component.decl;

			tokens.push_all(
				quote_tokens!(&*context, $decl).as_slice()
			);
		}

		tokens
	}
}
