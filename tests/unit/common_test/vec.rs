use common::physics::Vec2;


#[test]
fn it_should_return_the_zero_Vector() {
	assert_eq!(Vec2 { x: 0.0, y: 0.0 }, Vec2::zero())
}

#[test]
fn it_should_add_two_vectors() {
	let a = Vec2 { x: 1.0, y: 2.0 };
	let b = Vec2 { x: 2.0, y: 1.0 };

	assert!(a + b == Vec2 { x: 3.0, y: 3.0 });
}

#[test]
fn it_should_subtract_a_vector_from_another() {
	let a = Vec2 { x: 2.0, y: 3.0 };
	let b = Vec2 { x: 0.5, y: 1.0 };

	assert!(a - b == Vec2 { x: 1.5, y: 2.0 });
}

#[test]
fn it_should_scale_a_vector() {
	let v = Vec2 { x: 1.0, y: 2.0 };
	let s = 2.0;

	assert!(v * s == Vec2 { x: 2.0, y: 4.0 });
}

#[test]
fn it_should_compute_a_vectors_magnitude() {
	let v = Vec2 { x: 3.0, y: 4.0 };

	assert!(v.magnitude() == 5.0);
}

#[test]
fn it_should_normalize_a_vector() {
	let a = Vec2 { x: 3.0, y: 0.0 };
	let b = Vec2 { x: 0.0, y: 4.0 };

	assert!(a.normalize() == Vec2 { x: 1.0, y: 0.0 });
	assert!(b.normalize() == Vec2 { x: 0.0, y: 1.0 });
}

#[test]
fn it_should_round_a_vector() {
	let v = Vec2 { x: 0.25, y: 0.375 };

	assert_eq!(Vec2 { x: 0.25, y: 0.25 }, v.round(2));
}
