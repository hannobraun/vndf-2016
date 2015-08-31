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
use client::render::camera::{Camera,CameraTrack};
use shared::game::EntityId;

const DRAGMIN_PX: i32 = 5i32;      // arbitrary 5px minimum
const DRAGMIN_TIME: f64 = 0.30f64; // 75ms time minimum

#[derive(Debug)]
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
                   window_size: (u32,u32),
                   camera: &Camera) {
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
                        self.click = None;
                    }
                    else {
                        self.click = self.drag.0;
                        self.drag.0 = None;
                    }

                    self.handler(events,frame,window_size,camera);
                },
                MouseWheel(_) => { },
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
               window_size: (u32,u32),
               camera: &Camera) {
        if let Some(click) = self.click {
            //TODO: if no entity, pass on to UI (or viceversa)
            let coord = Mouse::convert_coord(click,window_size);
            let select = Mouse::check_selection(coord,frame,camera.get_pos());
            if let Some(id) = select {
                // TODO: consider conbining these two and handling selection
                // logic outside of this, in interface perhaps
                events.push(InputEvent::Select(vec!(id)));
                events.push(InputEvent::Track(CameraTrack::Entity(vec!(id))));
            }
        }
        else if let Some(drag_end) = self.drag.1 {
            let drag_start = self.drag.0.unwrap();
            let start = Mouse::convert_coord(drag_start,window_size);
            let end = Mouse::convert_coord(drag_end,window_size);

            let mut v = vec!();
            for ship in frame.ships.iter() {
                let ship_x = ship.1.position[0] as f32;
                let ship_y = ship.1.position[1] as f32;
                let cam_pos = camera.get_pos();
                let p = [ship_x + -(cam_pos[0]),ship_y + -(cam_pos[1])];

                
                if Mouse::within_bounds(p[0],start[0],end[0]) {
                    if Mouse::within_bounds(p[1],start[1],end[1]) {
                        v.push(ship.0.clone());
                    }
                }
            }

            events.push(InputEvent::Select(v.clone()));
            events.push(InputEvent::Track(CameraTrack::Entity(v))); 
        }
    }

    /// determines if point is within other points
    fn within_bounds(p: f32, start: f32, end: f32) -> bool {
        let mut within = false;
        if start < end {
            if (p > start) &
                (p < end) { within = true; }
        }
        else {
            if (p < start) &
                (p > end) { within = true; }
        }

        within
    }

    /// converts mouse coordinate to world position
    pub fn convert_coord(pos: (i32,i32), window_size: (u32,u32)) -> [f32;2] {
        let x = pos.0 - (window_size.0 as i32) /2;
        let y = pos.1 - (window_size.1 as i32) /2;

        [x as f32,y as f32]
    }

    // NOTE: assumes ships are equilateral triangles, & calcs bounding box
    // TODO: This is broken with a scale_factor != 1, as it assumes ships are
    //       sized 30 pixels wide and high. The actual ship size is stored in
    //       ShipDrawer. Maybe we can move it (together with font size, window
    //       size and other data) into some struct that can be accessed easily
    //       in all places that need that data.
    // we'll need to pass in mesh data eventually 
    fn check_selection(pos: [f32;2],
                           frame: &Frame,
                           cam_pos: [f32;2])
                           -> Option<EntityId> {
        for ship in frame.ships.iter() {
            let ship_x = ship.1.position[0] as f32;
            let ship_y = ship.1.position[1] as f32;
            if ((pos[0] + -(cam_pos[0])) < (ship_x + 15.0)) &
                ((pos[0] + -(cam_pos[0])) > (ship_x - 15.0)) &
                ((pos[1] + -(cam_pos[1])) < (ship_y + 15.0)) &
                ((pos[1] + -(cam_pos[1])) > (ship_y - 15.0))
            {
                return Some(ship.0.clone());
            }
        }

        None
    }
}
