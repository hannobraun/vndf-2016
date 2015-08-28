use std::env;


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
        // TODO: Execute client
        "client" => print!("client\n"),
        // TODO: Execute server
        "server" => print!("server\n"),
        // TODO: Execute tests
        "test"   => print!("test\n"),

        _ => print!("Unknown command: {}\n", command),
    }
}
