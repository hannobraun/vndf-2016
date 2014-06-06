use collections::HashMap;

use rustecs::{
	Components,
	EntityId,
};


struct MyComponentA { x: bool }
struct MyComponentB { x: bool }

entity!(MyEntity<MyComponentA, MyComponentB>, |arg: bool| {
	(MyComponentA { x: arg }, MyComponentB { x: !arg })
})


#[test]
fn it_should_create_an_entity() {
	let mut components_a: Components<MyComponentA> = HashMap::new();
	let mut components_b: Components<MyComponentB> = HashMap::new();

	let id = 42;
	MyEntity::create(id, true, &mut components_a, &mut components_b);

	assert_eq!(1, components_a.len());
	assert_eq!(1, components_b.len());

	assert_eq!(true , components_a.get(&id).x);
	assert_eq!(false, components_b.get(&id).x);
}
