use std::ascii::StrAsciiExt;
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

	var_name: ast::Ident,
	decl    : Vec<ast::TokenTree>,
	init    : Vec<ast::TokenTree>,
	insert  : Vec<ast::TokenTree>,
	remove  : Vec<ast::TokenTree>,
}

impl Component {
	fn generate(context: &ExtCtxt, component: &parse::Component) -> Component {
		let var_name = ast::Ident::new(
			token::intern(ident_to_lower(component.name).as_slice()));

		let collection = component.collection;
		let ty         = component.ty;

		let decl = quote_tokens!(&*context,
			pub $collection: ::rustecs::Components<$ty>,
		);

		let init = quote_tokens!(&*context,
			$collection: ::rustecs::components(),
		);

		let insert = quote_tokens!(&*context,
			self.$collection.insert(id, $var_name);
		);

		let remove = quote_tokens!(&*context,
			self.$collection.remove(&id);
		);

		Component {
			name    : token::get_ident(component.name).to_str(),
			var_name: var_name,
			decl    : decl,
			init    : init,
			insert  : insert,
			remove  : remove,
		}
	}
}


pub struct Entity {
	components: HashMap<String, Component>,
	create_fn : Vec<ast::TokenTree>,
}

impl Entity {
	fn generate(
		context       : &ExtCtxt,
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

		let ordered_components: Vec<String> = entity.components
			.iter()
			.map(|&ident|
				token::get_ident(ident).to_str())
			.collect();

		let create_fn = Entity::create_fn(
			context,
			entity,
			&entity_components,
			&ordered_components);

		Entity {
			components: entity_components,
			create_fn : create_fn,
		}
	}

	fn create_fn(
		context           : &ExtCtxt,
		entity            : &parse::Entity,
		components        : &HashMap<String, Component>,
		ordered_components: &Vec<String>
	) -> Vec<ast::TokenTree> {
		let name = ast::Ident::new(token::intern(
			"create_"
				.to_str()
				.append(ident_to_lower(entity.name).as_slice())
				.as_slice()));

		let mut args = Vec::new();
		for (i, arg) in entity.args.iter().enumerate() {
			if i + 1 < entity.args.len() {
				args.push_all(quote_tokens!(&*context, $arg,).as_slice());
			}
			else {
				args.push_all(quote_tokens!(&*context, $arg).as_slice());
			}
		}

		let mut component_names = Vec::new();
		for (i, name) in ordered_components.iter().enumerate() {
			let component = components.get(name);
			let var_name  = component.var_name;

			if i + 1 < components.len() {
				component_names.push_all(
					quote_tokens!(&*context, $var_name,).as_slice());
			}
			else {
				component_names.push_all(
					quote_tokens!(&*context, $var_name).as_slice());
			}
		}

		let init_block = entity.init_block;

		let mut inserts = Vec::new();
		for (_, component) in components.iter() {
			inserts.push_all(component.insert.as_slice());
		}

		quote_tokens!(&*context,
			pub fn $name(&mut self, $args) -> ::rustecs::EntityId {
				let id = self.next_id;
				self.next_id += 1;

				let ($component_names) = $init_block;

				$inserts

				id
			}
		)
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

		let name       = world.name;
		let decls      = World::component_decls(&components);
		let inits      = World::component_inits(&components);
		let create_fns = World::create_fns(entities);
		let removes    = World::removes(&components);

		let structure = quote_item!(&*context,
			pub struct $name {
				next_id: ::rustecs::EntityId,
				$decls
			}
		);

		let implementation = quote_item!(&*context,
			impl $name {
				pub fn new() -> $name {
					$name {
						next_id: 0,
						$inits
					}
				}

				$create_fns

				pub fn destroy_entity(&mut self, id: ::rustecs::EntityId) {
					$removes
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
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut tokens = vec!();

		for (_, component) in components.iter() {
			tokens.push_all(component.decl.as_slice());
		}

		tokens
	}

	fn component_inits(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut tokens = vec!();

		for (_, component) in components.iter() {
			tokens.push_all(component.init.as_slice());
		}

		tokens
	}

	fn create_fns(entities: &Vec<Entity>) -> Vec<ast::TokenTree> {
		let mut tokens = Vec::new();

		for entity in entities.iter() {
			tokens.push_all(entity.create_fn.as_slice());
		}

		tokens
	}

	fn removes(components: &HashMap<String, Component>) -> Vec<ast::TokenTree> {
		let mut removes = Vec::new();

		for (_, component) in components.iter() {
			removes.push_all(component.remove.as_slice());
		}

		removes
	}
}


fn ident_to_lower(ident: ast::Ident) -> String {
	token::get_ident(ident)
		.to_str()
		.as_slice()
		.to_ascii_lower()
}
