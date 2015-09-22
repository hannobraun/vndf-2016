use time::precise_time_s;
use nalgebra::{Vec2,cast};

use glutin::Event;
use glutin::ElementState;
use glutin::MouseButton;
use glutin::MouseScrollDelta;
use glutin::Event::{
    MouseMoved,
    MouseInput,
    MouseWheel,
};

use client::graphics::camera::{Camera};
use client::interface::{
    Frame,
    InputEvent,
};
use shared::game::EntityId;
use shared::physics::SphereCollider;

const DRAGMIN_PX: i32 = 5i32;      // arbitrary 5px minimum
const DRAGMIN_TIME: f64 = 0.30f64; // 30ms time minimum

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
                   camera: &mut Camera) {
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
                MouseWheel(d) => {
                    match d {
                        MouseScrollDelta::LineDelta(_,y) => {
                            camera.zoom(y as f32 / 3.5);
                        },
                        MouseScrollDelta::PixelDelta(_,y) => {
                            camera.zoom(y as f32 / 3.5); //pixeldelta might need tweaking
                        },
                    }
                },
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
        let cam_pos: Vec2<f32> = cast(camera.get_pos());
        if let Some(click) = self.click {
            //TODO: if no entity, pass on to UI (or viceversa)
            let coord = Mouse::convert_coord(click,window_size);

	    let adj_pos: Vec2<f64> = cast(coord * camera.zoom
					  + (cam_pos * -1.0));
	    let select = Mouse::check_selection(adj_pos,
						frame,
						camera.zoom);
	    
            if let Some(id) = select {
                if !frame.select_ids.contains(&id) {
                    events.push(InputEvent::Select(vec!(id)));
                }
                else {
                    events.push(InputEvent::Deselect(vec!(id)));
                }
            }
        }
        else if let Some(drag_end) = self.drag.1 {
            let drag_start = self.drag.0.unwrap();
            let start = Mouse::convert_coord(drag_start,window_size) * camera.zoom;
            let end = Mouse::convert_coord(drag_end,window_size) * camera.zoom;
            let start = start + (cam_pos * -1.0);
            let end = end + (cam_pos * -1.0);
            
            let mut v = vec!();
	    
            for (id,ship) in frame.ships.iter() {
                let ship_pos: Vec2<f32> = cast(ship.position);
                if Mouse::within_bounds(ship_pos[0],start[0],end[0]) {
                    if Mouse::within_bounds(ship_pos[1],start[1],end[1]) {
                        v.push(id.clone());
                    }
                }
            }

	    for (id,planet) in frame.planets.iter() {
                let planet_pos: Vec2<f32> = cast(planet.body.position);
                if Mouse::within_bounds(planet_pos[0],start[0],end[0]) {
                    if Mouse::within_bounds(planet_pos[1],start[1],end[1]) {
                        v.push(id.clone());
                    }
                }
            }

            if v.len() > 0 { //don't select nothing in a drag-select
                events.push(InputEvent::Select(v.clone()));
            }
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
    // TODO: consider taking camera position into account
    pub fn convert_coord(pos: (i32,i32), window_size: (u32,u32)) -> Vec2<f32> {
        let x = pos.0 - (window_size.0 as i32) /2;
        let y = pos.1 - (window_size.1 as i32) /2;

        Vec2::new(x as f32,-1.0*y as f32)
    }

    /// check the position against all known colliders
    fn check_selection(pos: Vec2<f64>,
                       frame: &Frame,
		       zoom: f32,)
                       -> Option<EntityId> {
	// NOTE: to make sure we select ships first, we should iterate over them manually
	// if we make changes to frame so that ships and planets becomes Bodies like server,
	// then we'll need to change this and basically collect planets during iteration
	// then iter over them after we check all ships
	
	// TODO: remove ships from iteration of which are not on the screen/in view;
	// see within_bounds function

	for (id,body) in frame.ships.iter() {
	    if let Some(coll) = frame.colliders.get(&id) {
                // update collider on the spot to reflect new position
                //SphereCollider::update(coll,&body.position); // TODO: implement this when ncollide updates
                
		if SphereCollider::check_pos(coll, &cast(pos), zoom*10.0) {
		    return Some(*id)
		}
	    }
	    else { warn!("no collider found for ship id {}", id); }
	}

	for (id,planet) in frame.planets.iter() {
	    if let Some(coll) = frame.colliders.get(&id) {
                // update collider on the spot to reflect new position
                //SphereCollider::update(coll,&planet.body.position);
                
		if SphereCollider::check_pos(&coll, &cast(pos), zoom*10.0) {
		    return Some(*id)
		}
	    }
	    else { warn!("no collider found for planet id {}", id); }
	}

        None
    }
}
