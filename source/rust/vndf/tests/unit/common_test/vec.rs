use vndf::physics::Vec2;


#[test]
fn it_should_add_two_vectors() {
	assert_eq!(
		Vec2(3.0, 3.0),
		Vec2(1.0, 2.0) + Vec2(2.0, 1.0));
}

#[test]
fn it_should_subtract_a_vector_from_another() {
	assert_eq!(
		Vec2(1.5, 2.0),
		Vec2(2.0, 3.0) - Vec2(0.5, 1.0));
}

#[test]
fn it_should_scale_a_vector() {
	assert_eq!(
		Vec2(2.0, 4.0),
		Vec2(1.0, 2.0) * 2.0);
}

#[test]
fn it_should_compute_a_vectors_magnitude() {
	assert_eq!(
		5.0,
		Vec2(3.0, 4.0).mag());
}

#[test]
fn it_should_normalize_a_vector() {
	assert_eq!(
		Vec2(1.0, 0.0),
		Vec2(3.0, 0.0).normalize());
	assert_eq!(
		Vec2(0.0, 1.0),
		Vec2(0.0, 4.0).normalize());
}

#[test]
fn it_should_round_a_vector() {
	assert_eq!(
		Vec2(0.25, 0.25),
		Vec2(0.25, 0.375).round(2));
}
