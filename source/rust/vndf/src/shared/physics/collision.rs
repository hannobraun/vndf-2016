use nalgebra::{Pnt2,Vec2};
use ncollide::shape::{Ball,Convex};
use ncollide::bounding_volume::{BoundingSphere,
                                HasBoundingVolume,
                                BoundingVolume};

use client::graphics::SHIP_SIZE;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub enum CollideKind {
    Ship(Convex<Pnt2<f32>>),
    Planet(Ball<f32>),
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
	let size = SHIP_SIZE/2.0 * scaling_factor;
	let p = vec![Pnt2::new(-0.5, -0.5) * size,
		     Pnt2::new( 0.5, -0.5) * size,
		     Pnt2::new( 0.0,  0.5) * size,];
	let c = Convex::new(p);
	
	Collider::new(CollideKind::Ship(c))
    }

    pub fn new_from_planet (planet_size: f32, scaling_factor: f32) -> Collider {
	let size = planet_size/2.0 * scaling_factor;
	let b = Ball::new(size);
	
	Collider::new(CollideKind::Planet(b))
    }

    /// requires two colliders and their associated positions in the world
    pub fn check_collision (&self,
                            pos: &Vec2<f32>,
                            other: (&Collider,&Vec2<f32>))
                            -> bool {
        let (other_kind, other_pos) = (&other.0.kind,&other.1);
	let mut is_collide = false;
	match (&self.kind, other_kind) {
	    (&CollideKind::Ship(ref c1), &CollideKind::Ship(ref c2)) => {
                let c1_b = c1.bounding_volume(pos);
                let c2_b: BoundingSphere<Pnt2<f32>> = c2.bounding_volume(*other_pos);
                is_collide = c2_b.intersects(&c1_b);
	    },
	    (&CollideKind::Ship(ref c1), &CollideKind::Planet(ref c2)) => {
                let c1_b = c1.bounding_volume(pos);
                let c2_b: BoundingSphere<Pnt2<f32>> = c2.bounding_volume(*other_pos);
                is_collide = c2_b.intersects(&c1_b);
	    },
	    _ => { warn!("Unsupported collision types"); }
	}

	is_collide
    }
}
