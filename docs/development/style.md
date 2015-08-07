## All Languages

### Line Length

Line length should be limited to 80 characters.


### Commit Messages

Git commit messages should help yourself and others to easily detect what kind of changes a specific commit introduced. To facilitate this, we try to follow these guidelines:

* The first line of the message should be
  * at most 50 characters long,
  * summarize the change as well as possible,
  * be written in a present-tense imperative voice ("Fix this", "Amend that to do whatever" instead of "Fixing this" or "Amended this to do whatever")
  * and not end with punctuation.
* The second line should be blank.
* After that, the change should be explained in as much details as makes sense. Each line should be at most 72 characters long.

Following this format makes sure the commit messages are well-formatted when viewing them in common tools (git log, gitk).

Your explanation should not only explain what has changed in as much detail as is sensible, but also your motivations for making this change. If any external (to the version control repository) resources are relevant to the change, mention those too.

If the commit is very small (as commits ideally should be) it is often clear from the diff what actually happened, and an explanation can be redundant. In that case, a one-word commit message like "Refactor" can be appropriate.

* Issues on GitHub can be referenced by adding the issue number (e.g. #4) to the commit message. This will add a reference to the commit to the issue automatically.
* Issues can be closed with a commit message like this: Close #5.
* Any other external resources should be linked using a normal URL.


## Rust

### TODO

To mark a defect in the code, please use a comment starting with "TODO: "
(without the quotes). Please use this exact spelling, including case and spaces.
This makes it easier to search for TODOs throughout the project.

When to use TODO:
* A piece of code is broken right now and needs to be fixed.
* A piece of code will need to be changed, when an _already scheduled_ task is
  going to be implemented.

When *not* to use TODO:
* For something that would be "nice to have".
* For something that might break in the future, if some theoretical change were
  to be made.

Please be serious about these guidelines. Adding TODOs to the code is very
useful to mark the exact piece of code that needs to change, which can help
tremendously. However, I've seen far too many codebases that are littered with
TODOs everywhere. When having too many of them, they lose all meaning, and
developers will start to ignore them.

If a piece of code is not as nice as it could be, or could break under some
circumstances, please add a normal comment explaining the situation, without a
TODO. If that ever becomes relevant, the developer working on the problem will
find your comment and can act accordingly. Quite likely (if my experience is any
indication) these things rarely become relevant at all.

#### Linking TODOs to tasks

If a TODO refers to a task on Trello, you should link the both of them. Here's
how you do that:

1. Open the Trello card and look at its URI. Some of the URI will be
   human-readable, but there will also be something that looks like an ID.
1. For example, if the URI is https://trello.com/c/I9XldTAu/26-reorganize-documentation, the ID is "I9XldTAu".
1. Add that id to your TODO. Instead of "TODO: ", write "TODO(I9XldTAu): ".
1. Mention in the card description that there are linked TODOs.
1. Optional: Push the linked TODOs to GitHub and add actual link to the TODOs
   into the card description.

This is a bit tedious, and that part about adding links to the Trello card
should _really_ be automated, but it has an important advantage: When starting
to work on a task, you can quickly see where to start and what will break due to
that task being implemented. This can save quite a lot of searching around and
also can prevent some ugly bug that would otherwise pop up, due to some code
that should have been adapted to the new reality.

As a general rule, there should, at any time, only be a few unlinked tasks. If
too many TODOs pile up in the code, just fix a few of them right away and link
others to cards on Trello.

#### Examples

Unlinked TODO:
> TODO: This will crash, if the user sends an empty string.

Linked TODO:
> TODO(I9XldTAu): This documentation is out of date.

To find the Trello task that belongs to this TODO, go to:
> https://trello.com/c/I9XldTAu

To find unlinked TODOs, search for "TODO:", e.g.:
> rgrep TODO: source/rust

To find TODOs belonging to a specific task, search for its id, e.g.:
> rgrep I9XldTAu source/rust

To find all TODOs, search for "TODO", e.g.
> rgrep TODO source/rust


### Imports

Imports (`use` statements in Rust) should be organized in such a way that makes them easy to understand when taking a quick look at them.

The following guidelines help with this:
* Group `use` statement into several blocks, each separated by a single space.
* Within a block, imports should be sorted alphabetically.

The groups:
1. Imports from libraries that are bundled with Rust (usually only `std`).
1. Imports from external libraries imported via Cargo.
1. Imports from the code within our repository.


## Bash

### Shebang

Starting a bash script with the proper shebang helps programs to recognize the file as a bash script. This is useful for executing the script on a wide range of operating systems, as well as other issues like syntax highlighting in Sublime Text.

Every bash script's first line should look like this:

> \#!/usr/bin/env bash
