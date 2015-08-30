use time::precise_time_s;

use glutin::Event;
use glutin::ElementState;
use glutin::MouseButton;
use glutin::Event::{
    MouseMoved,
    MouseInput,
    MouseWheel,
};

use client::interface::{
    Frame,
    InputEvent,
};

const DRAGMIN_PX: i32 = 5i32;      // arbitrary 5px minimum
const DRAGMIN_TIME: f64 = 75f64; // 75ms time minimum

pub struct Mouse {
    pos: (i32,i32),
    drag: (Option<(i32,i32)>,Option<(i32,i32)>),
    drag_start: f64,
    click: Option<(i32,i32)>,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            pos: (0,0),
            drag: (None,None),
            drag_start: precise_time_s(),
            click: None,
        }
    }
    pub fn update (&mut self,
                   events: &mut Vec<InputEvent>,
                   frame : &Frame,
                   window_events: &Vec<Event>,
                   window_size: (u32,u32)) {
        for event in window_events.iter() {
            match *event {
                MouseMoved(pos) => {
                    self.pos = pos;
                },
                MouseInput(ElementState::Pressed,MouseButton::Left) => {
                    self.drag.0 = Some(self.pos);
                    self.drag_start = precise_time_s();
                },
                MouseInput(ElementState::Released,MouseButton::Left) => {
                    if ((precise_time_s()-self.drag_start) > DRAGMIN_TIME) &
                        (((self.drag.0).unwrap().0 - self.pos.0).abs() >
                        DRAGMIN_PX) &
                        (((self.drag.0).unwrap().1 - self.pos.1).abs() >
                        DRAGMIN_PX)
                    {
                        self.drag.1 = Some(self.pos);
                    }
                    else {
                        self.click = self.drag.0;
                        self.drag.0 = None;
                    }

                    self.handler(events,frame,window_size);
                },
                MouseWheel(d) => { },
                _ => { },
            }
        }
    }
    pub fn is_dragging (&self) -> bool {
        self.drag.0.is_some()
    }
    
    pub fn get_drag(&mut self) -> Option<((i32,i32),(i32,i32))> {
        if let Some(s) = self.drag.0 {
            if let Some(e) = self.drag.1 {
                let drag = Some((s,e));
                self.drag = (None,None);
                drag
            }
            else { None }
        }
        else { None }
    }

    pub fn get_click(&mut self) -> Option<(i32,i32)> {
        let click = self.click;
        self.click = None;
        click
    }

    fn handler(&mut self,
               events: &mut Vec<InputEvent>,
               frame : &Frame,
               window_size: (u32,u32)) {
        if let Some(click) = self.click {
            //TODO: find entity that was clicked
            //if no entity, pass on to UI (or viceversa)
            let coord = Mouse::convert_coord(click,window_size);
            println!("{:?}",coord);
        }
        else if let Some(drag_end) = self.drag.1 {
            let drag_start = self.drag.0.unwrap();
            //TODO: find entities that were selected
        }
    }

    /// converts mouse coordinate to world position
    pub fn convert_coord(pos: (i32,i32), window_size: (u32,u32)) -> [f32;2] {
        let x = pos.0 - (window_size.0 as i32) /2;
        let y = pos.1 - (window_size.1 as i32) /2;

        [x as f32,y as f32]
    }
}
