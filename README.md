# Von Neumann Defense Force

## About This Repository

This repository holds the source code for Von Neumann Defense Force, a multiplayer game that was in development for a number of years, but was ultimately abandoned. The game was created by [Hanno Braun](https://github.com/hannobraun), who was later joined by [Chris Gill](https://github.com/viperscape).

This is the repository that was used from the very beginning, containing thousands of commits, from the [very first one](https://github.com/hannobraun/vndf/commit/aeb419ea66cf03a783bdd18b4ef4b6ce6980f9a8), through all the twists and turns, all the failures and successes, on to the last whimpers and final abandonment. We're releasing it in the hope that you may use the code for something useful.

Please note that the code in this repository is that of an in-progress game project that started years ago and was abandoned mid-development. As such, the code was never really intended for public consumption and is of varying quality.


## Getting Started

This game is written in [Rust](https://www.rust-lang.org/). To work on it (or run it), you need the following prerequisites:
- A Rust nightly from around December 2015 (the last time this game was being worked on). I recommend using [rustup](http://rustup.rs) to install it.
- [FreeType](http://www.freetype.org/) is required to run and to build the
  client. On Linux, it should be available via your package manager. If you have
  installed it, but still get a link error, you might also need to install the
  matching `-dev` package.

Once you're set up with Rust and Cargo, you can build Pan, the command-line "IDE" we used to help with development.

On Linux:

> ./setup.sh

On Windows:

> ./setup.bat

Once Pan is set up, you can use it to do some standard development tasks.

Run all tests:

> pan test

Start the server:

> pan server

Start a client:

> pan client

To successfully start a client, you need to run the server first, so the client has something to connect to (see above).


## Additional Information

The VNDF website is located at [http://hannobraun.de/vndf](http://hannobraun.de/vndf). Especially interesting is the [newsletter archive](http://hannobraun.de/vndf/news/).

There's also some documentation in the [docs directory](https://github.com/hannobraun/vndf/tree/master/docs) of this repository. The [game design](https://github.com/hannobraun/vndf/blob/master/docs/game-design/start-here.md) documentation is probably most interesting.

And of course there's the [source code](https://github.com/hannobraun/vndf/tree/master/source/rust/vndf) itself.


## License

Unless noted otherwise, all code in this repository is licensed under the conditions of the [CC0](https://creativecommons.org/publicdomain/zero/1.0/), which basically makes it available to everyone, without any restrictions. Please refer to the [summary](https://creativecommons.org/publicdomain/zero/1.0/) and the [full legal text](https://creativecommons.org/publicdomain/zero/1.0/legalcode).
