use std::collections::{HashMap,BTreeMap};

use rustc_serialize::json::{
    self,
    DecodeResult,
};

use shared::game::{
    Body,
    EntityId,
};
use client::graphics::camera::CameraTrack;


#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Frame {
    pub ship_id     : Option<EntityId>,
    pub game_time_s : Option<f64>,
    pub message     : Message,
    pub broadcasts  : HashMap<EntityId, String>,
    pub ships       : BTreeMap<EntityId, Body>,
    pub camera_track: Option<CameraTrack>,
    pub select_ids  : Vec<EntityId>,
    pub deltatime   : f64, // time between last frame and this frame
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            ship_id     : None,
            game_time_s : None,
            message     : Message::None,
            broadcasts  : HashMap::new(),
            ships       : BTreeMap::new(),
            camera_track: Some(CameraTrack::Default),
            select_ids  : Vec::new(),
            deltatime   : 0.0,
        }
    }

    pub fn from_json(json: &str) -> DecodeResult<Frame> {
        json::decode(json)
    }

    pub fn to_json(&self) -> String {
        match json::encode(self) {
            Ok(encoded) => encoded,
            Err(error)  => panic!("Encoding error: {}", error)
        }
    }
}


#[derive(Clone, Debug, RustcDecodable, RustcEncodable, Eq, PartialEq)]
pub enum Message {
    Notice(String),
    Error(String),
    None,
}

impl Message {
    pub fn is_notice(&self) -> bool {
        if let &Message::Notice(_) = self {
            true
        }
        else {
            false
        }
    }

    pub fn is_error(&self) -> bool {
    if let &Message::Error(_) = self {
        true
    }
    else {
        false
    }
}

pub fn is_none(&self) -> bool {
if let &Message::None = self {
    true
}
else {
    false
}
        }
}
