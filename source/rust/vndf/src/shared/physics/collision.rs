use nalgebra::{Pnt2,Vec2};
use ncollide::shape::{Ball,Convex};
use ncollide::bounding_volume::{bounding_sphere,
				BoundingVolume,};

use client::graphics::SHIP_SIZE;

// TODO: Consider removing collidekind and going with BoundingSphere only;
// this means that if we wanted to have higher accuracy (odd shape sizes)
// then we'd need this back, at a minimum
#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub enum CollideKind {
    Ship(Convex<Pnt2<f64>>),
    Planet(Ball<f64>),
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Collider {
    kind: CollideKind,
}

impl Collider {
    pub fn new (kind: CollideKind) -> Collider {
	Collider { kind: kind, }
    }

    /// builds based on current ship mesh layout (from equilateral triangle)
    // TODO: make ship mesh points as public in shapes module
    pub fn new_from_ship (scaling_factor: f32) -> Collider {
	let size = (SHIP_SIZE/2.0 * scaling_factor) as f64;
	let p = vec![Pnt2::new(-0.5, -0.5) * size,
		     Pnt2::new( 0.5, -0.5) * size,
		     Pnt2::new( 0.0,  0.5) * size,];
	let c = Convex::new(p);
	
	Collider::new(CollideKind::Ship(c))
    }

    pub fn new_from_planet (planet_size: f32, scaling_factor: f32) -> Collider {
	let size = (planet_size/2.0 * scaling_factor) as f64;
	let b = Ball::new(size);
	
	Collider::new(CollideKind::Planet(b))
    }

    /// requires two colliders and their associated positions in the world
    pub fn check_collision (&self,
                            pos: &Vec2<f64>,
                            other: (&Collider,&Vec2<f64>))
                            -> bool {
        let (other_kind, other_pos) = (&other.0.kind,other.1);
	let mut is_collide = false;
	match (&self.kind, other_kind) {
	    (&CollideKind::Ship(ref c1), &CollideKind::Ship(ref c2)) => {
                let c1_b = bounding_sphere(c1,pos);
                let c2_b = bounding_sphere(c2,other_pos);
                is_collide = c2_b.intersects(&c1_b);
	    },
	    (&CollideKind::Ship(ref c1), &CollideKind::Planet(ref c2)) => {
		let c1_b = bounding_sphere(c1,pos);
                let c2_b = bounding_sphere(c2,other_pos);
                is_collide = c2_b.intersects(&c1_b);
	    },
	    _ => { warn!("Unsupported collision types"); }
	}

	is_collide
    }
}
