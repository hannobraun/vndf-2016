use nalgebra::{Vec2,Absolute};
use shared::game::EntityId;
use client::interface::Frame;

/// Camera tracking types
#[derive(Debug,Clone,RustcDecodable,RustcEncodable,PartialEq)]
pub enum CameraTrack {
    Entity(Vec<EntityId>),
    Position,
    Default,
}

pub struct Camera {
    track: CameraTrack,
    pos: Vec2<f64>,
    speed: f64, // camera transition speed
    // TODO: consider camera easing
}

impl Camera {
    pub fn new () -> Camera {
        Camera {
            track: CameraTrack::Position,
            pos: Vec2::new(0.0,0.0),
            speed: 5.0,
        }
    }

    pub fn set (&mut self, tracking: CameraTrack) {
        self.track = tracking;
    }
    
    /// must be called to update camera positioning
    pub fn update (&mut self,
                   frame: &Frame,
                   offset: Option<Vec2<f64>>)
                   -> Vec2<f64> {
        let mut pos = Vec2::new(0.0,0.0);
        let mut vel = Vec2::new(0.0,0.0);
        
        match self.track {
            CameraTrack::Entity(ref v) => {                
                pos = Camera::get_average_pos(&v,&frame);
            },
            CameraTrack::Default => { 
                if let Some(id) = frame.ship_id {
                    self.track = CameraTrack::Entity(vec!(id));
                }
            },
            _ => (),
        }
        
        if let Some(offset) = offset {
            pos = pos+offset;
        }

        // NOTE: must invert each coordinate to track
        pos[0] *= -1.0;
        pos[1] *= -1.0;
        
        //TODO: make pos a Vec2 and figure out Absolute trait
        let abs_x = (self.pos[0] - pos[0]).abs();
        let abs_y = (self.pos[1] - pos[1]).abs();

        if abs_x > (self.speed/2.0) {
            let mut factor_x = self.speed;

            // flip direction for camera
            if self.pos[0] > pos[0] { factor_x *= -1.0; }
            self.pos[0] += factor_x;
        }
        else {
            self.pos[0] = pos[0];
        }

        if abs_y > (self.speed/2.0) {
            let mut factor_y = self.speed;

            // flip direction for camera
            if self.pos[1] > pos[1] { factor_y *= -1.0; }
            self.pos[1] += factor_y;
        }
        else {
            self.pos[1] = pos[1];
        }
        
        self.pos
    }

    /// gets the average position of multiple entities
    // NOTE: This assumes that frame will hold all entities (eg: ships & planets)
    pub fn get_average_pos (v: &Vec<EntityId>, frame: &Frame) -> Vec2<f64> {
        let mut ax = 0.0;
        let mut ay = 0.0;
        let total = v.len() as f64;
        
        // for now grab ships
        for n in v.iter() {
            if let Some(b) = frame.ships.get(&n) {
                ax += b.position[0];
                ay += b.position[1];
            }
        }

        Vec2::new((ax/total), (ay/total))
    }

    pub fn get_pos (&self) -> Vec2<f64> {
        self.pos
    }
}
