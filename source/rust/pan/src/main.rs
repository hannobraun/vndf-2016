#![feature(path_ext)]


use std::env::{
    self,
    Args,
};
use std::fs::PathExt;
use std::path::PathBuf;
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


    // TODO: Those paths are redundantly specified in project.conf
    const BINARY_PATH: &'static str = "output/cargo/debug";
    const RUST_PATH  : &'static str = "source/rust/vndf";

    let mut still_searching = true;
    let mut root_path = env::current_dir().expect("Expected current directory");
    while still_searching {
        let candidate = root_path.join("project.conf");

        if candidate.exists() {
            still_searching = false;
        }
        else {
            if !root_path.pop() {
                panic!("Could not find VNDF repository root");
            }
        }
    }

    let paths = Paths {
        binaries   : root_path.join(BINARY_PATH),
        rust_source: root_path.join(RUST_PATH),
    };


    match command {
        "client" => build_and_run("vndf-client", args, paths, "vndf_client=trace"),
        "server" => build_and_run("vndf-server", args, paths, "trace"),
        "test"   => run_tests(paths),

        _ => print!("Unknown command: {}\n", command),
    }
}


struct Paths {
    binaries   : PathBuf,
    rust_source: PathBuf,
}


fn run_tests(paths: Paths) {
    let path = format!(
        "{}:{}",
        env::var("PATH").unwrap_or_else(|e| panic!("Environment error: {}", e)),
        paths.binaries.display(),
    );

    run_command(
        Command::new("cargo")
            .arg("test")
            .current_dir(paths.rust_source)
            .env("PATH", path)
            .env("RUST_LOG", "vndf_server=info,vndf_client=info")
            .env("RUST_BACKTRACE", "1")
    );
}


fn build_and_run(binary: &str, args: Args, paths: Paths, log_config: &str) {
    run_command(
        Command::new("cargo")
            .args(&["build", "--bin", binary])
            .current_dir(paths.rust_source)
    );

    let mut command = Command::new(paths.binaries.join(binary));
    for arg in args {
        command.arg(arg);
    }
    command.env("RUST_LOG", log_config);

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
