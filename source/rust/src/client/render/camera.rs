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
    pos: [f32;2],
}

impl Camera {
    pub fn new () -> Camera {
        Camera {
            track: CameraTrack::Position,
            pos: [0.0,0.0],
        }
    }

    pub fn set (&mut self, tracking: CameraTrack) {
        self.track = tracking;
    }
    
    /// must be called to update camera positioning
    pub fn update (&mut self,
                   frame: &Frame,
                   offset: Option<[f32;2]>)
                   -> [f32;2] {
        let mut pos = [0.0,0.0];
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
            pos[0] += offset[0];
            pos[1] += offset[1];
        }

        // NOTE: must invert each coordinate to track
        pos[0] *= -1.0;
        pos[1] *= -1.0;
        
        self.pos = pos;
        pos
    }

    /// gets the average position of multiple entities
    // NOTE: This assumes that frame will hold all entities (eg: ships & planets)
    pub fn get_average_pos (v: &Vec<EntityId>, frame: &Frame) -> [f32;2] {
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

        [(ax/total) as f32, (ay/total) as f32]
    }

    pub fn get_pos (&self) -> [f32;2] {
        self.pos
    }
}
