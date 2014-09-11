# Ongoing Cleanup Work

The work listed in here is low priority: Nice to have long-term, but not really
that problematic. Nice to do on the side, whenever you need something less
complicated to relax in between more demanding tasks.

If the mess to be cleaned up is actually affecting day-to-day work in any
meaningful capacity, it should be a task in the tracker.


## Split vndf package

Optimally, the vndf package should only contain binary crates, the acceptance
test suite and possibly a library crate (not sure if that makes sense, we'll
see). All the rest should be extracted into separate packages to improve
incremental compile times.
