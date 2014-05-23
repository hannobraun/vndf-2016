# Von Neumann Defense Force

## Prerequisites

The game is currently developed on Ubuntu 14.04 exclusively. Other Linux systems
will probably work, but other operating systems are not supported and it might
be significant work to get the dev environment working on them.


## Resources

The source repository for this project is hosted on GitHub:
https://github.com/hannobraun/vndf

We also use GitHub's issue tracking and wikie:
* Issues: https://github.com/hannobraun/vndf/issues
* Wiki: https://github.com/hannobraun/vndf/wiki


## Initialize the Repository

After cloning the repository, you need to initialize it:

> ./scripts/init

This should take care of the rough stuff, however, it's likely that some things
you need to actually run the scripts in the repository will not be installed.
Feel free to amend the script accordingly or file issues on the tracker.


## Start the Continous Testing Loop

> ./scripts/test

This will start a continuous testing loop that will re-compile the project and
re-run the tests on every change.

As I'm writing this, there are still some minor problems with the script, but
it's being worked on. If you run into issues, just hit CTRL-C and restart it.


## Start the Server-side Components

> ./scripts/server

This will compile and start the server-side components. It will also re-compile
and re-start them, if you make changes to the source.

At the time of writing, this is in a state of disrepair. In time, this will be
fixed with the help of Zwobot, but for now, you can start the game service
manually like this:

> output/bin/vndf-game-service

If that file doesn't exist, create it by running scripts/test.


## Start the Client

> ./scripts/client

This requires a server to run (see above).


## Deploy to the production environment

At the time of writing, this function has moved into the utility-server
repository.
