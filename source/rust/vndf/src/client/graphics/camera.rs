use std::collections::HashSet;
use nalgebra::{Vec2,Translation,Norm};
use shared::game::EntityId;
use client::interface::Frame;

/// Camera tracking types
#[derive(Debug,Clone,RustcDecodable,RustcEncodable,PartialEq)]
pub enum CameraTrack {
    Entity(HashSet<EntityId>),
    Position,
    Default,
}

pub struct Camera {
    track: CameraTrack,
    pos: Vec2<f64>,
    vel: Vec2<f64>,
    speed: f64, // camera transition speed
    pub zoom: f32,
    //time: f64,
}

const MAX_ZOOM: f32 = 1000.0;

impl Camera {
    pub fn new () -> Camera {
        Camera {
            track: CameraTrack::Position,
            pos: Vec2::new(0.0,0.0),
            vel: Vec2::new(0.0,0.0),
            speed: 0.3,
            zoom: 1.0,
            //time: precise_time_s(),
        }
    }

    pub fn set_track (&mut self, tracking: CameraTrack) {
        self.track = tracking;
    }

    /// speed for camera easing, must be within 0.1 and 1.0
    pub fn set_speed (&mut self, speed: f64) {
        let mut t = speed;
        
        // clamp speed
        if t > 1.0 { t = 1.0; }
        if t < 0.1 { t = 0.1; }
        
        self.speed = t;
    }

    pub fn zoom (&mut self, z: f32) {
        self.zoom += z;
        if self.zoom > MAX_ZOOM {
            self.zoom = MAX_ZOOM;
        }
        if self.zoom < 1.0 {
            self.zoom = 1.0;
        }
    }
    
    /// must be called to update camera positioning
    pub fn update (&mut self,
                   frame: &Frame)
                   -> Vec2<f64> {
        let mut pos = Vec2::new(0.0,0.0);
        let mut vel = Vec2::new(0.0,0.0);
        
        match self.track {
            CameraTrack::Entity(ref v) => {                
                let (p,v) = Camera::get_average_pos(&v,&frame);
                pos = p;
                vel = v;
            },
            CameraTrack::Default => { 
                if let Some(id) = frame.ship_id {
                    let mut set = HashSet::new();
                    set.insert(id);
                    self.track = CameraTrack::Entity(set);
                }
            },
            _ => (),
        }

        // NOTE: must invert each coordinate to track
        pos = pos.inv_translation();
        vel = vel.inv_translation();
        self.vel = vel; //track velocity
        
        if (pos-(self.pos+self.vel)).sqnorm() > 10.0 { //removes jittering
            let d = pos-self.pos;
            let mut t = 1.0/d.sqnorm().sqrt(); // get vector magnitude
            t += self.speed;
            
            // clamp speed
            if t > 1.0 { t = 1.0; }
            if t < 0.1 { t = 0.1; }
            
            self.pos = (self.pos*(1.0-t)) + (pos*t); // ease out
        }
        else { self.pos = pos; } //removes jittering

        //let dt = precise_time_s() - self.time;
        //self.time = dt;
        self.pos
    }

    /// gets the average position of multiple entities
    // NOTE: This assumes that frame will hold all entities (eg: ships & planets)
    pub fn get_average_pos (v: &HashSet<EntityId>, frame: &Frame) -> (Vec2<f64>,Vec2<f64>) {
        let mut pos = Vec2::new(0.0,0.0);
        let mut vel = Vec2::new(0.0,0.0);
        let total_ships = v.len() as f64;
        let total = Vec2::new(total_ships,total_ships);
        
        // for now grab ships
        for n in v.iter() {
            if let Some(b) = frame.ships.get(&n) {
                pos = pos + b.position;
                vel = vel + b.velocity;
            }
        }

        (pos/total,
         vel/total)
    }

    pub fn get_pos (&self) -> Vec2<f64> {
        self.pos
    }
}
