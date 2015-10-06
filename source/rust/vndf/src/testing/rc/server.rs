use server::game::initial_state::InitialState;
use testing::process::Process;
use testing::util::{
    random_path,
    random_port,
};


pub struct Server {
    port    : u16,
    _process: Process,
}

impl Server {
    pub fn start(initial_state: InitialState) -> Server {
        let initial_state_file = random_path();
        initial_state.to_file(&initial_state_file);

        let port = random_port(40000, 50000);

        let mut process = Process::start(
            "vndf-server",
            &[
                format!("--port={}"          , port              ).as_ref(),
                format!("--client-timeout={}", 0.1               ).as_ref(),
                format!("--sleep-duration={}", 5                 ).as_ref(),
                format!("--initial-state={}" , initial_state_file).as_ref(),
            ]
        );
        process.read_stderr_line(); // Make sure it's ready

        Server {
            port    : port,
            _process: process,
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
