use std::env;
use std::process::{
    Command,
    Stdio,
};


fn main() {
    let mut args = env::args();

    let _program = args.next();
    let command  = args.next();

    let command = match command {
        Some(ref command) =>
            command.as_ref(),
        None => {
            print!("You need to provide an argument.\n");
            return;
        }
    };

    match command {
        "client" => run_client(args),
        // TODO: Execute server
        "server" => print!("server\n"),
        // TODO: Execute tests
        "test"   => print!("test\n"),

        _ => print!("Unknown command: {}\n", command),
    }
}


fn run_client<I>(args: I) where I: Iterator<Item=String> {
    let status =
        Command::new("cargo")
            .args(&["build", "--bin", "vndf-client"])
            // TODO: Read path from configuration file
            .current_dir("source/rust/vndf")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap_or_else(|e| panic!("Error running client build: {}", e));

    if !status.success() {
        panic!("Client build exited with status {}", status);
    }

    // TODO: Read path from configuration file
    let mut command = Command::new("output/cargo/debug/vndf-client");
    for arg in args {
        command.arg(arg);
    }

    let status =
        command
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap_or_else(|e| panic!("Error running client: {}", e));

    if !status.success() {
        panic!("Client exited with status {}", status);
    }
}
