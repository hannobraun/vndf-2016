use shared::game::EntityId;
use client::interface::Frame;

/// Camera tracking types
#[derive(Debug,Clone,Copy,RustcDecodable, RustcEncodable)]
pub enum CameraTrack {
    Entity(EntityId),
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
            CameraTrack::Entity(eid) => {
                if let Some(ref ship) = frame.ships.get(&eid) {
                    // NOTE: neg(x) will keep position, this gets flipped for some reason
                    pos = [-1.0f32 * ship.position.x as f32,
                           -1.0f32 * ship.position.y as f32];
                }
            },
            CameraTrack::Default => { 
                if let Some(id) = frame.ship_id {
                    self.track = CameraTrack::Entity(id);
                }
            },
            _ => (),
        }

        if let Some(offset) = offset {
            pos[0] += offset[0];
            pos[1] += offset[1];
        }

        self.pos = pos;
        pos
    }
}
