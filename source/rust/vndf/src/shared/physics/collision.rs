use nalgebra::{Vec2};
use ncollide::shape::{Ball};
use ncollide::bounding_volume::{bounding_sphere,
                                BoundingVolume,
                                BoundingSphere};

// Only deal with Ball<f32>
pub type Sphere = BoundingSphere<Ball<f32>>; // precomputed bounding sphere

pub struct SphereCollider;

impl SphereCollider {
    pub fn new_from_oval (radius: f32) -> Ball<f32>  {
        let b = Ball::new(radius);
        b

            // we'll return this once ncollide supports boundingsphere in hashmap
            //SphereCollider::as_sphere(&b,&Vec2::new(0.0,0.0))
    }

    /*/// rebuilds convex to pre-computed bounding sphere
    pub fn as_sphere (c: &Ball<f32>, pos: &Vec2<f32>) -> Sphere {
        bounding_sphere(c,pos)
    }*/

    /*/// updates an existing boundingsphere
    pub fn update (bs: &mut Sphere,
                   pos: &Vec2<f32>,
                   zoom: f32) {
        *bs = BoundingSphere::new(*bs.center() + *pos,
                                  bs.radius() *  zoom)
    }*/

    /// checks if position is inside
    pub fn check_pos (b: &Ball<f32>,
                      pos: &Vec2<f32>,
                      zoom: f32,)
                      -> bool {

        let bs1 = bounding_sphere(b,pos);
        let bs2 = bounding_sphere(&Ball::new(1.0*zoom),pos);
        bs1.intersects(&bs2)
    }

    /// checks for collision
    // TODO: integrate zooming
    pub fn check_collision (c1: (&Ball<f32>, &Vec2<f32>),
                            c2: (&Ball<f32>, &Vec2<f32>),)
                            -> bool {

        let bs1 = bounding_sphere(c1.0,c1.1);
        let bs2 = bounding_sphere(c2.0,c2.1);
        bs1.intersects(&bs2)
    }
}
