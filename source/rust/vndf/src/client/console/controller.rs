use std::collections::HashMap;

use glutin::VirtualKeyCode;
use glutin::ElementState::Pressed;
use glutin::Event;
use glutin::Event::{
    KeyboardInput,
    ReceivedCharacter,
};

use client::console::Console;
use client::interface::{
    Frame,
    InputEvent,
    Message,
};
use shared::game::{ManeuverData,EntityId};

pub struct Controller {
    pub console: Console,

    height      : u16,

    cmd_history: Vec<String>,
    cmd_history_idx: usize, //history index/cursor
    is_cmd_history: bool,
    tmp_cmd_buffer: String, //used to temporarily store typed characters

    comm_cache: HashMap<EntityId, String>, // cache of all broadcasts, to be compared
    comm_subscribe: bool,
}

impl Controller {
    pub fn new() -> Controller {
        let mut text = Vec::new();
        text.push(format!("VNDF Ship Control System"));
        text.push(format!("Type \"help\" to list available commands"));

        let height = 24;

        Controller {
            console: Console::new(text),

            height      : height,
            cmd_history: vec!(),
            cmd_history_idx: 0,
            is_cmd_history: false,
            tmp_cmd_buffer: String::new(),

            comm_cache: HashMap::new(),
            comm_subscribe: true,
        }
    }

    pub fn update(
        &mut self,
        events: &mut Vec<InputEvent>,
        frame : &Frame,
        window_events: &Vec<Event>,
            ) {
        match frame.message {
            Message::Notice(ref message) => self.console.output.push(format!("Notice: {}", message)),
            Message::Error(ref message)  => self.console.output.push(format!("Error: {}", message)),
            Message::None                => (),
        }

        for event in window_events.iter() {
            match *event {
                ReceivedCharacter(c) =>
                    if !c.is_control() {
                        self.console.input.insert(self.console.prompt_index,c);
                        self.is_cmd_history = false;
                        self.console.prompt_index +=1;
                    },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Left)) => {
                    if self.console.prompt_index > 0 { self.console.prompt_index -= 1; }
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Right)) => {
                    if self.console.prompt_index < self.console.input.chars().count()  {
                        self.console.prompt_index += 1;
                    }
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Home)) => {
                    self.console.prompt_index = 0;
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::End)) => {
                    self.console.prompt_index = self.console.input.chars().count();
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Delete)) => {
                    let byte_index = self.console.input
                        .char_indices()
                        .nth(self.console.prompt_index);
                    if let Some((byte_index, _)) = byte_index {
                        self.console.input.remove(byte_index);
                    }
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Back)) => {
                    if self.console.prompt_index > 0 {
                        self.console.prompt_index -= 1;

                        let byte_index = self.console.input
                            .char_indices()
                            .nth(self.console.prompt_index);
                        if let Some((byte_index, _)) = byte_index {
                            self.console.input.remove(byte_index);
                        }
                    }

                    self.is_cmd_history = false;
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Return)) => {
                    let command = self.console.input.clone();
                    
                    if command != "" {
                        let mut found = (false,0);
                        for (i,_cmd) in self.cmd_history.iter().enumerate() {
                            if *_cmd == command { found = (true,i); }
                        }

                        if found.0 { self.cmd_history.swap_remove(found.1); }
                        
                        self.cmd_history.insert(0,command.clone());
                        
                        self.console.input.clear();
                        self.tmp_cmd_buffer.clear();
                        self.is_cmd_history = false;
                        self.cmd_history_idx = 0; //optionally reset idx
                        self.console.prompt_index = 0;

                        self.handle_line(
                            events,
                            command.as_ref(),
                            frame,
                            );
                    }
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Up)) => {
                    let cmd = self.get_history(true);
                    self.console.input.clear();
                    self.console.input.push_str(&cmd);
                    self.console.prompt_index = cmd.chars().count();
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Down)) => {
                    let cmd = self.get_history(false);
                    self.console.input.clear();
                    self.console.input.push_str(&cmd);
                    self.console.prompt_index = cmd.chars().count();
                },
                _ => (), // ignore other events
            }
        }

        while self.console.output.len() > (self.height - 2) as usize {
            self.console.output.remove(0);
        }

        // check for new broadcasts
        // NOTE: does not display re-broadcasted text that is exactly the same
        for (id,msg) in &frame.broadcasts {
            let mut new_comm = false;
            if let Some(m) = self.comm_cache.get(id) {
                if msg != m {
                    new_comm = true;
                }
            }
            else { // entirely new broadcaster
                new_comm = true;
            }

            if new_comm & self.comm_subscribe {
                self.comm_cache.insert(*id,msg.clone());
                if let Some(my_id) = frame.ship_id {
                    if my_id != *id {
                        self.console.output.push(format!("{}: {}", id, msg));
                    }
                }
            }
        }
    }

    fn get_history (&mut self, rev: bool) -> String {
        if self.cmd_history.len() == 0 { return "".to_string() }
        
        //shift cursor based on direction
        if rev { //heading to past commands
            if !self.is_cmd_history  { //first time pulling up history?
                self.tmp_cmd_buffer = self.console.input.clone();
            }
            else if self.cmd_history_idx < (self.cmd_history.len()-1) {
                self.cmd_history_idx += 1;
            }
        }
        else {
            if self.is_cmd_history { //shifting within cmd history already?
                //head to more recent cmds
                if self.cmd_history_idx > 0 {
                    self.cmd_history_idx -= 1;
                }
                else { //already at most recent history-command in buffer
                    self.is_cmd_history = false;
                    let _tmp = self.tmp_cmd_buffer.clone();
                    return _tmp
                }
            }
            else {
                return self.tmp_cmd_buffer.clone()
            }
        }

        self.is_cmd_history = true;
        self.cmd_history[self.cmd_history_idx].clone()
    }

    pub fn get_prompt_idx(&self) -> usize {
        self.console.prompt_index
    }
    
    pub fn text(&self) -> &[String] {
        self.console.output.as_ref()
    }

    pub fn input(&self) -> &str {
        self.console.input.as_ref()
    }

    fn handle_line(
        &mut self,
        events: &mut Vec<InputEvent>,
        line  : &str,
        frame : &Frame
            ) {
        self.console.output.push(format!("> {}", line));

        let mut splits = line.splitn(2, ' ');

        let command = splits.next().unwrap();
        let args    = splits.next().unwrap_or("");

        match command {
            "list-broadcasts" => {
                self.console.output.push(format!("{} broadcasts", frame.broadcasts.len()));
                for (eid,msg) in &frame.broadcasts {
                    self.console.output.push(format!("{}: {}", eid, msg));
                }
            },
            "start-broadcast" => {
                events.push(InputEvent::StartBroadcast(args.to_string()));
            },
            "stop-broadcast" => {
                events.push(InputEvent::StopBroadcast);
            },

            "comm-subscribe" => {
                self.comm_subscribe = true;
            },

            "comm-ignore" => {
                self.comm_subscribe = false;
            },

            "nav-data" => {
                match frame.ship_id {
                    Some(ship_id) => {
                        let ship = frame.ships[&ship_id];

                        self.console.output.push(format!(
                            "Position: ({}, {}); Velocity: ({}, {})\n",
                            ship.position.x, ship.position.y,
                            ship.velocity.x, ship.velocity.y,
                            ));
                    },
                    None => {
                        self.console.output.push(format!("No data available."));
                    },
                }
            },
            "comm-data" => {
                let message = match frame.ship_id {
                    Some(ref id) => format!("Your Comm Id: {}", id),
                    None         => format!("Comm Id is currently unknown"),
                };
                
                self.console.output.push(message);
            },

            "schedule-maneuver" => {
                let result = scan_fmt!(
                    args,
                    "{} {} {}",
                    f64, f64, f64
                        );

                match result {
                    (Some(delay_s), Some(duration_s), Some(direction_deg)) => {
                        let direction_rad = (direction_deg as f64).to_radians();

                        let game_time_s =
                            if let Some(game_time_s) = frame.game_time_s {
                                game_time_s
                            }
                        else {
                            self.console.output.push(format!(
                                "{} {}",
                                "Server connection not fully established ",
                                "yet. Please try again in a moment."
                                    ));
                            return;
                        };

                        // This uses the game time from the last frame as the
                        // base time. If this ever turns out to be too
                        // imprecise, we could count the time since the last
                        // frame arrived.
                        let data = ManeuverData {
                            start_s   : game_time_s + delay_s,
                            duration_s: duration_s,
                            angle     : direction_rad,
                        };

                        events.push(
                            InputEvent::ScheduleManeuver(data)
                                )
                    },
                    _ =>
                        self.console.output.push(format!("Error parsing arguments")),
                }
            },
            "list-maneuvers" => {
                self.console.output.push(format!("Scheduled maneuvers:"));
                for (_, maneuver) in &frame.maneuvers {
                    self.console.output.push(format!(
                        "Start: {}; Duration: {}; Angle: {}",
                        maneuver.start_s,
                        maneuver.duration_s,
                        maneuver.angle,
                    ));
                }
            },
            
            "select-entity" => {
                let ents = Controller::parse_entity_ids(args);
                events.push(InputEvent::Select(ents));
            },

            "deselect-entity" => {
                let ents = Controller::parse_entity_ids(args);
                events.push(InputEvent::Deselect(ents));
            },

            "clear-selection" => {
                events.push(InputEvent::Deselect(vec!()));
            },
            
            "help" => {
                let help = [
                    "list-broadcasts - Lists all received broadcasts",
                    "start-broadcast <text> - Start sending a broadcast",
                    "stop-broadcast - Stop sending the current broadcast",
                    "comm-subscribe/ignore - actively list or ignore new broadcasts",
                    "nav-data - Print navigation data",
                    "comm-data - Print communication data",
                    "schedule-maneuver <delay (s)> <duration (s)> <degrees> - Schedule a maneuver",
                    "list-maneuvers - List all scheduled maneuvers",
                    "select-entity <list of ship_id, separate by space>",
                    "clear-selection - Clears currently selected entities",
                    ];

                self.console.output.push(format!("Available commands:"));
                for line in &help {
                    self.console.output.push(format!("{}", line))
                }
            }

            _ => self.console.output.push(format!("Unknown command: {}\n", command)),
        }
    }

    /// parses entity ids from args
    /// does not check if entityid currently exists
    pub fn parse_entity_ids(args: &str) -> Vec<EntityId> {
        let args = args.split(' ');
        let mut ents = vec!();
        
        for n in args {
            if let Some(id) = n.parse::<EntityId>().ok() {
                ents.push(id);
            }
        }

        ents
    }
}
