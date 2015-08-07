## Implement a solution that's good enough for now

Most problems have a lot of different solutions. Some of those solutions are perceived as being more ideal than others. Don't fall for the temptation of trying to implement those. The ideal solution is often a lot of work. Maybe all this work will pay off. It's a lot more likely though that a simpler solution would have gotten you the same results much faster.

* Implement something that's good enough for now.
* Another good expression to keep in mind is, "the simplest thing that could possibly work".
* When tempted to solve problems that we might have in the future, remember the YAGNI principle: You Ain't Gonna Need It.

This rule is meant to get us to deliver fast results. Too often programmers waste their time on solving theoretical problems that will never be encountered in practice, all the while deceiving themselves into thinking they're doing useful work.

However, this rule should never be used to encourage or justify sloppy work that will lead to all kinds of problems later. Limitations should be obvious or at least become obvious once they are relevant.

* Never write insecure code in services that are exposed to the public.
* Never write sloppy code that might lead to subtle and hard to find bugs, if it is used in a different context later on.
* However, it's perfectly fine to just crash the program with an informative error message, if the code is used in a way it wasn't designed for.


## Add incremental value

Making big changes is complicated and risky.

* The smaller a change, the better you or someone else can understand it.
* Small changes provide you with feedback earlier, allowing you to correct course, if something doesn't work out as expected.

A big change that is half done is just wasted effort. If you're interrupted and can't complete it, it just sits there, unfinished. Coming back to it later, you'll probably no longer understand it completely. Or maybe the code you were going to change has been changed by someone else in the meantime, making your half-done work now obsolete.

* Think about how you can split your big task into tiny changes that each add value individually.
* Even if you never get to finish your task, the changes you already made, maybe cleaning up existing code before modifying the way it works, have already improved what was there.
* You or others can more easily pick up an interrupted task later, if what's already been done has been split into tiny, understandable changes, each with its own commit message explaining it.

It's usually possible to split a change into tiny steps. However, it's often not possible for each of these steps to be an improvement. Don't be afraid to make the code more messy and complicated, if you have to. As long as is brings you closer to your goal, this is still preferable to large, all-or-nothing changes.


## Work on what's important

Both adding new features and fixing existing issues are very important activities. Prioritizing one over the other will either lead to a horribly broken product or will halt all progress.

* Always prioritize bugs over everything else. If a feature just doesn't work correctly, that is worse than not having the feature at all.
* Things that could be better (this could be easier, that could do more) are not bugs though! We categorize those as enhancements in the issue tracker. Balance your work on those issues with working on new features.
* I recommend alternating between the two: After you've implemented a new feature, why not take some time to work on some issues? That keep forward progress going, quality high and allows you to keep your mind fresh by switching it up from time to time.


## Automate everything

Doing something that the computer can do for you is a waste of your time.

* If you do something a second time, think about automating it right away.
* If it's not trivial to automate, create a card on Trello.

Every time you repeat a task that is not automated or only partially automated, note this on the issue tracker. This helps you and others to better judge how important this task is, compared to other issues.


## Test everything (automatically)

Unless it's absolutely necessary to keep things going, never change any program behaviour without writing a test for that new behaviour first.

* If you're adding a new feature, write an automated test to test that feature.
* If you're changing an existing feature, there should be an existing test for it. Modify that.
* Don't optimize code without writing a benchmark to back you up.
* Code should always be as simple and clear as possible. If you're going to make it more complicated, you need to justify that by showing the benefit with an automated test or benchmark!

## Don't be afraid to make a mess

Sometimes it is unclear how a problem should be solved. In these situations it is easy to fall into analysis paralysis, not making any progress until you manage to look at your problem from another perspective.

Of course it is worth thinking about hard problems, but if that doesn't yield any results, it can be beneficial to just barge ahead to achieve the desired outcome, no matter how much of a mess you're making while doing it. Doing this can often help you understand your problem better, which helps you arrive at a better solution.

* When writing new code, don't be afraid to make a mess, if it helps you to make progress. It can always be cleaned up later!
* However, be careful of not overdoing it. If everyone just makes messes and no one ever cleans up, the code becomes a nightmare quite fast.
* To counteract this rule, remember to always leave code better than you found it. It's okay to make a mess of new code, if necessary. The same goes for existing code that requires significant changes to achieve a goal. But never make existing code messier without a good reason! You should never allow small hacks and short-cuts to accumulate on top of once-decent code.


## Leave code better than you found it

When modifying existing code, always make sure to leave it better than you found it.

* Was something hard to understand? Think about how you can make it simpler!
* Is the style inconsistent? Clean it up!

Leaving code better than you found it guarantees that the code will improve in the long run, instead of becoming harder and harder to work with over time.


## Use comments sparsely

A lot of people advocate to thoroughly comment code. I believe the opposite:
Code should be commented as sparsely as possible. Add enough comments, so that
it still makes sense, but no more.

Reasons:
* Comments age and get out of date. They often get ignored when the code around
  them is updated, making them plain wrong. Code with misleading comments is
  harder to understand than code without any comments at all.
* If you only have few comments, the comments you have will stand out. If you
  comment everything, nobody can tell what can be ignored and what is actually
  important.
* Code should be clear and speak for itself. Clear code with few comments is
  easier to understand than unclear code with a lot of comments.

### Writing cleaner code

Whole books have been written about this (for example Clean Code, which I
recommend). Here's a simple example on how to remove comments while actually
making the code clearer.

Commented code:
```
// Do A
this;
is;
all;
gibberish;

// Do B
more;
gibberish;
let x = something;

// Do C
if x {
	more;
	stuff;
	nobody;
	understands;
}
```

Cleaner code:

```
do_a();
let x = do_b();
if x {
	do_c();
}


fn do_a() {
	this;
	is;
	all;
	gibberish;
}

fn do_b() -> bool {
	more;
	gibberish;

	something
}

fn do_c() {
	more;
	stuff;
	nobody;
	understands;
}
```

This is much better. The cleaned up code with the function calls gives you a
good overview over what happens, as the function names carry just as much
information as the comments did. Plus, function names don't get out-of-date as
easily as comments do.

The hard-to-understand code is isolated into short functions which, by their
name, clearly tell what they do. You could clean this up further by splitting
the hard-to-understand code further up, into even smaller functions.

### When comments are appropriate

Add a comment whenever the code does something that is not obvious by looking at
it, and you can't make that thing obvious by changing the code.

An example:
```
if something_is_the_case {
	do_whatever();
}
else {
	// We don't need to do anything in this case, because of that other thing
	// that this comment explains fully.
}
```

Why nothing needs to be done in the `else` case might not be obvious to a
reader, so an empty `else` case with a clarifying comment can help a lot.
