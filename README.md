# Von Neumann Defense Force

## Prerequisites

To work on VNDF you need two things:
- Bash, to run the scripts in the `scripts/` directory. On Windows,
  [Cygwin](https://www.cygwin.com/) should work.
- An up-to-date installation of [Rust](https://www.rust-lang.org/). I recommend
  [multirust](https://github.com/brson/multirust) to manage your installation.


## Resources

The source repository for this project is hosted on GitHub:
https://github.com/hannobraun/vndf

Documentation is available in the
`[docs](https://github.com/hannobraun/vndf/tree/master/docs)/` directory.

Project management is done on
[Trello](https://trello.com/b/WdwuT2Fx/von-neumann-defense-force).


## Run all tests

> ./scripts/test

This will compile the project and run all tests. Please do this regularly,
especially before pushing code to GitHub.


## Start the server

> ./scripts/server

This will compile the project and run a server process.


## Start the client

> ./scripts/client

This will start the client. You need to run a server first, for the client to
connect to (see above).


## Deploy to the production environment

This function has moved to another repository, because VNDF shares a server with
other projects. Once VNDF gets a dedicated server, we'll move the deployment
infrastructure back here.
