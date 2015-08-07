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

* Issues on GitHub can be referenced by adding the issue number (e.g. #4) to the commit message. This will add a reference to the commit to the issue automatically.
* Issues can be closed with a commit message like this: Close #5.
* Any other external resources should be linked using a normal URL.


## Bash

### Shebang

Starting a bash script with the proper shebang helps programs to recognize the file as a bash script. This is useful for executing the script on a wide range of operating systems, as well as other issues like syntax highlighting in Sublime Text.

Every bash script's first line should look like this:

> \#!/usr/bin/env bash
