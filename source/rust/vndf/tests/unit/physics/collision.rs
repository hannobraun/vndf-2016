use nalgebra::Vec2;
use vndf::shared::physics::collision::SphereCollider;
use vndf::client::graphics::SHIP_SIZE;

#[test]
fn collision() {
    let c1 = SphereCollider::new_from_oval(SHIP_SIZE);
    let c2 = SphereCollider::new_from_oval(SHIP_SIZE);

    let planet_size = 30.0; //diameter
    let planet_rad = planet_size/2.0;
    let c3 = SphereCollider::new_from_oval(planet_size);
    
    let origin = Vec2::new(0.0,0.0);
    
    assert!(SphereCollider::
            check_collision((&c1,&origin),
			    (&c2,&origin)));

    assert!(!SphereCollider::
            check_collision((&c2,&origin),
			    (&c1,&Vec2::new(SHIP_SIZE,SHIP_SIZE))));

    assert!(SphereCollider::
            check_collision((&c2,&origin),
			    (&c3,&Vec2::new(planet_rad,planet_rad))));

    assert!(!SphereCollider::
            check_collision((&c2,&origin),
			    (&c3,&Vec2::new(planet_size,planet_size))));
}
