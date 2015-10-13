use nalgebra::Vec2;
use vndf::shared::physics::collision::SphereCollider;
use vndf::client::graphics::SHIP_SIZE;

#[test]
fn collision() {
    let c1 = SphereCollider::new_from_oval(SHIP_SIZE / 2.0);
    let c2 = SphereCollider::new_from_oval(SHIP_SIZE / 2.0);

    let planet_diameter = 30.0;
    let planet_radius = planet_diameter/2.0;
    let c3 = SphereCollider::new_from_oval(planet_radius);
    
    let origin = Vec2::new(0.0,0.0);
    
    assert!(SphereCollider::
            check_collision((&c1,&origin),
                (&c2,&origin)));

    assert!(!SphereCollider::
            check_collision((&c2,&origin),
                (&c1,&Vec2::new(SHIP_SIZE,SHIP_SIZE))));

    assert!(SphereCollider::
            check_collision((&c2,&origin),
                (&c3,&Vec2::new(planet_radius,planet_radius))));

    assert!(!SphereCollider::
            check_collision((&c2,&origin),
                (&c3,&Vec2::new(planet_diameter,planet_diameter))));
}
