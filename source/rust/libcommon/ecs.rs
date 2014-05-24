use collections::HashMap;

pub type EntityId      = uint;
pub type Components<T> = HashMap<EntityId, T>;


pub trait EntityTemplate2<A, C1, C2> {
	fn create(&self, args: A, c1: &mut Components<C1>, c2: &mut Components<C2>);

	fn destroy(&self,
		id: EntityId, c1:
		&mut Components<C1>,
		c2: &mut Components<C2>) {

		c1.remove(&id);
		c2.remove(&id);
	}
}
