use collections::HashMap;

pub type EntityId      = uint;
pub type Components<T> = HashMap<EntityId, T>;


pub trait EntityTemplate2<A, C1, C2> {
	fn create(&self,
		id     : EntityId,
		args   : A,
		comps_1: &mut Components<C1>,
		comps_2: &mut Components<C2>) {

		let (c1, c2) = self.create_components(args);
		comps_1.insert(id, c1);
		comps_2.insert(id, c2);
	}

	fn destroy(&self,
		id: EntityId, c1:
		&mut Components<C1>,
		c2: &mut Components<C2>) {

		c1.remove(&id);
		c2.remove(&id);
	}

	fn create_components(&self, args: A) -> (C1, C2);
}
