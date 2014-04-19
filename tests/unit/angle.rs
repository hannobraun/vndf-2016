use std::f64;

use common::physics::{Radians, Vec2};


#[test]
fn it_should_convert_from_a_vector(){
	assert_eq!(
		Radians(0.0),
		Radians::from_vec(Vec2 { x: 1.0, y: 0.0}));
	assert_eq!(
		Radians(f64::consts::PI / 2.0),
		Radians::from_vec(Vec2 { x: 0.0, y: 1.0}));
}
