use rustc_serialize::json::{
    self,
    DecodeResult,
};

use client::graphics::camera::CameraTrack;
use shared::game::{ManeuverData,EntityId};

#[derive(Clone, Debug, RustcDecodable, RustcEncodable, PartialEq)]
pub enum InputEvent {
    StartBroadcast(String),
    StopBroadcast,

    ScheduleManeuver(ManeuverData),

    Track(CameraTrack), // sets camera tracking
    
    Select(Vec<EntityId>),
    Deselect(Vec<EntityId>),
    
    Quit,
}

impl InputEvent {
    pub fn from_json(json: &str) -> DecodeResult<InputEvent> {
        json::decode(json)
    }

    pub fn to_json(&self) -> String {
        match json::encode(self) {
            Ok(encoded) => encoded,
            Err(error)  => panic!("Encoding error: {}", error)
        }
    }
}
