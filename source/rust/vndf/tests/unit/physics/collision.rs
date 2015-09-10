use nalgebra::Vec2;
use vndf::shared::physics::collision::Collider;

#[test]
fn collision() {
    let c1 = Collider::new_from_ship(1.0);
    let c2 = Collider::new_from_ship(1.0);

    let planet_size = 30.0; //diameter
    let planet_rad = planet_size/2.0;
    let c3 = Collider::new_from_planet(planet_size, 1.0);
    
    let origin = Vec2::new(0.0,0.0);
    
    assert!(c1.check_collision(&origin,
			       (&c2,&origin)));

    assert!(!c2.check_collision(&origin,
				(&c1,&Vec2::new(16.0,16.0))));

    assert!(c2.check_collision(&origin,
			       (&c3,&Vec2::new(planet_rad,planet_rad))));

    assert!(!c2.check_collision(&origin,
			       (&c3,&Vec2::new(planet_size,planet_size))));
}
