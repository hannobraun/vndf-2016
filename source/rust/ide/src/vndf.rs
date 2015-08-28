use std::env::{
    self,
    Args,
};
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
        "client" => build_and_run("vndf-client", args),
        "server" => build_and_run("vndf-server", args),
        "test"   => run_tests(),

        _ => print!("Unknown command: {}\n", command),
    }
}


fn run_tests() {
    let path = format!(
        "{}:{}",
        env::var("PATH").unwrap_or_else(|e| panic!("Environment error: {}", e)),
        // TODO: Read path from configuration file
        "output/cargo/debug",
    );

    run_command(
        Command::new("cargo")
            .arg("test")
            // TODO: Read path from configuration file
            .current_dir("source/rust/vndf")
            .env("PATH", path)
            .env("RUST_LOG", "vndf_server=info")
            .env("RUST_BACKTRACE", "1")
    );
}


fn build_and_run(binary: &str, args: Args) {
    run_command(
        Command::new("cargo")
            .args(&["build", "--bin", binary])
            // TODO: Read path from configuration file
            .current_dir("source/rust/vndf")
    );

    // TODO: Read path from configuration file
    let binary = format!("output/cargo/debug/{}", binary);

    let mut command = Command::new(binary);
    for arg in args {
        command.arg(arg);
    }
    command.env("RUST_LOG", "trace");

    run_command(&mut command);
}

fn run_command(command: &mut Command) {
    let status =
        command
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap_or_else(|e| panic!("Error running {:?}: {}", command, e));

    if !status.success() {
        panic!("{:?} exited with status {}", command, status);
    }
}
