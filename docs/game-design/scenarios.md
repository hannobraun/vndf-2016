# Scenarios

Description of scenarios that a player might experience while playing the game.
The point of this document is to show the higher-level concepts introduced in
the other documents in action and describe the game in a concrete way that is
really hard to do by presenting high-level concepts.

A few notes before I start:

- All of this is very preliminary. I'm very sure that no scenario will make it
  into the game just as I describe it here. Lots will change, as we gather
  experience.
- Scenarios are partially speculative, so I can paint a fuller picture. For
  example, I don't really know what the tutorial will look like, but I have to
  start somewhere to describe the game.


## New Player Experience

- A player logs into the game for the first time. Their ship is slowly moving
  through space.
- The player ship is represented by a symbol, next to it is text that represents
  the most important information about the ship. This information includes the
  current position and velocity.

OPEN QUESTION: What other information is displayed next to the ship?

- The camera is centered on the player ship, so even though the ship moves, it
  stays in the middle of the screen.
- Extending from the player ship to the right is a prominent, slightly curved
  line that represents the ship's predicted course.
- In the background is a regular grid. The camera is zoomed in closely, so the
  grid cells are scaled to match a relatively small distance in space, like
  1 km.
- The grid has a constant position in the coordinate system, which means it is
  slowly moving on the screen, as the camera follows the player ship.

OPEN QUESTION: Is the grid labelled to show units? It would make sense to have
that information. Maybe subtle position markings at each line intersection?

- The rest of the user interface represents the ship's systems and the actions
  the player can take.

NOTE: I'm leaving out the details now, as a new player wouldn't understand
anyway and it's much easier to explain them in context, when those sytems and
actions become relevant.

OPEN QUESTION: Does the player see the full UI? It might be sense to start only
with the most basic systems and activate the rest over time during a tutorial.

- The player zooms out, using the mouse wheel or a keyboard shortcut. As they
  zoom out, the ship's symbol and the text next to it keep the same size.
- As the camera zooms out and more of the grid is shown, the grid cells become
  smaller.
- Before cells become too small, most of the grid lines fade out, to make larger
  cells.

NOTE: For example, if 3 out of 4 grid lines faded out (which means every 4th
line would stay exactly the same), units of 4x4 grid cells would be converted
into a single, larger cell.

- This fading out of small cells repeats until the zooming is completed.
- As the player zooms out, the predicted course is extended, so it always
  extends to the edge of the screen. As more of it is seen, the curvature
  becomes more pronounced.
- As the player zooms out, a moon comes into view. Contrary to ships and other
  small objects, large celestial objects are represented as circles that cover
  their whole extent.
- As the player zooms out and the moon comes into view, the predicted path
  becomes a circle and it becomes clear that the player's ship is in orbit
  around the moon. Since we can see the full orbit now, the predicated path no
  longer leaves the screen, but ends at the players symbol.

OPEN QUESTION: I described the predicted path at first as only originating from
the ship symbol and forming a circle only when the full orbit into view. Maybe
it should have been a full circle (that we first only saw a tiny part of) from
the beginning? The variation I described should be easier to implement (just
keep computing more path segments until you reach the edge of the screen, or the
player ship), while the "full circle" option would need to detect somehow that
the player is in an orbit.

FUTURE EXTENSION: When we see a full circle orbit, we need some indication into
what direction the player is moving. Maybe the path could be color-coded somehow
to show when the ship will reach a part of the path. Color-coding paths would
also help to instantly judge if ships on intersecting paths come near each
other. Same or similar color at the intersection? They will come close.

- The player sees no other objects besides their own ship and the moon. The
  player decides to start a scan with their sensors, to see if something else is
  in the vicinity.

NOTE: I describe the player as just knowing to do stuff to keep things simple.
In the actual game, this would be embedded into some kind of tutorial.

- The player clicks a button to activate the ship's RADAR. This sends out
  signals that could be sensed by other players (which makes it potentially
  dangerous), but the player is not concerned about this right now.

OPEN QUESTION: Whether this button is always visible or whether there is some
kind of tab system remains to be seen.

OPEN QUESTION: Are new players within some kind of instanced area until they
complete the first parts of the tutorial?

- After a few moments, results start coming in. The ship's sensors detect other,
  so far unidentified objects in orbit around the moon. Those are represented by
  a symbol that is distinct from the ship symbol.
- The objects are located on the player's side of the moon, as the moon shields
  everything behind it from detection by the player.
- Next to those symbols is textual information about the objects: Their
  position, velocity, size, mass, energy emission, and composition.
- As the objects are mostly composed of metal and emit little energy, the player
  concludes that these are either wrecks or inactive ships. The player is
  optimistic and judges that those are wrecks, not ships lying in wait.
- The player selects a few nearby objects by clicking on them. The selected
  object's symbols are somehow marked as being selected.
- For the selected objects, the predicted paths are displayed. Like the player's
  path, those form an orbit around the moon.
- The player chooses one object they can easily reach and deselects the others.
- The player determines they need raise their ship's orbit to intersect the
  object on the other side of the moon. By clicking on their predicated path
  shortly before them, they open a piece of user interface that is located where
  they clicked. They use this UI to schedule a maneuver that will be executed
  when the ship reaches that point.

OPEN QUESTION: How exactly will this UI look? Kerbal Space Program's maneuver
nodes are an obvious inspiration.

NOTE: An introduction on orbital mechanics is out of scope for this document. We
will have to address this for the game's tutorial of course, but until then I
recommend playing Kerbal Space Program and/or watching KSP tutorial videos on
YouTube.

FUTURE EXTENSION: It may be a good idea to give the players additional tools for
maneuver planning. For example, they could select a new orbit by dragging with
the mouse and that tool could then schedule the required maneuvers to reach the
new orbit.

- The ship's projected path immediately reflects the planned maneuver, as the
  player adjusts the maneuver. This gives the player direct feedback about the
  effect of the planned maneuver.
- After they're satisfied with the maneuver, the player immediately schedules a
  second maneuver on the other side of the moon, where the ship will rendevouz
  with the target object. The purpose of the second maneuver is to adjust the
  ship's velocity to match the target object's, so both will float right next to
  each other.
- The player waits a moment until the ship reaches the point of the scheduled
  maneuver. Once it arrives, the maneuver is being executed. The player can see
  the velocity next to the ship symbol adjust.
- Now the player waits until their ship reaches the other side of the moon,
  where it will rendevouz with the object.

NOTE: We will have to choose the attributes of planets such, that this waiting
around doesn't become boring. For example, the planets in KSP are much denser
then the densest material in the real world, to reduce the time it takes to
orbit them.

- As the ships comes closer to the target object, the sensors become able to
  identify it. It turns out to be a large part of debris from a destroyed ship.
- This information is reflected by the symbol of the object changing to a symbol
  that denotes debris and ship wrecks. The text next to the object also changes
  to show the important information for this type of object.

OPEN QUESTION: What information is most important for which types of object
remains to be determined.

- The player's ship reaches the location of the second maneuver, next to the
  piece of debris. The ship executes the maneuver. The ship and the piece of
  debris now float right next to each other, with only a hundred meters
  distance.
- The ship for new players is quite small compared to other ships, but any ship
  with a certain payload (weapons, cargo capacity, other systems) that is
  capable of interplanetary flight still weighs hundreds of tons, making it too
  sluggish to navigate closely to the debris. The player decides to dispatch
  some drones to break the debris into small parts and load the pieces into the
  ship's cargo hold.
- The drones are small spaceships that are remote-controlled by the player's
  ship. They have an extremly short range and are reliant on the main ship, but
  they have some basic tools to break apart resources and carry them to the
  player's ship.
- Not every ship would have such drones aboard, but the ships for new players
  are designed to be versatile.
- The piece of debris is already selected, so all the player has to do is click
  the button that send out the drones to do resource gathering.

FUTURE EXTENSION: Presumable there would be more UI than a single button, to
configure how many drones to send out and such.

- Piece by piece, the drones load the metal from the debris into the ship's
  cargo hold. After a while, the whole piece is broken apart and the player's
  cargo hold is full.

FUTURE EXTENSION: Eventually we might want a sophisticated resource system, but
in the beginning we should keep it simple and have one resource, "metal", to
build things out of.

- With the cargo hold full, but the fuel tanks looking not so good, there is not
  much the player can still do here. They decide to fly to the planet the moon
  orbits, which is settled, and serves as a trade hub.
- The player zooms out yet again, until they see the planet the moon orbits. The
  ship, the moon and the other objects around the moon that the player
  discovered earlier, now occupy the same few pixels on the screen. To give a
  good overview of what's happening, the game shows the symbol for a celestial
  object (the moon), with the symbol for the player ship, and the symbol for
  multiple unknown objects smaller next to it.

NOTE: Presumably, we would have to develop some kind of symbol language. For
example, the symbol for "many unknown objects" would be arranged in the same way
as the symbol for "many enemy ships". The symbol for "a collection of different
objects" would be composed of the symbols for the different parts of objects
that part of the collection.

- Similar to how they did before, the player schedules two maneuvers. One to get
  the ship out of the moons orbit and towards the planet, another to decelerate
  to enter orbit once near the planet.
- After the first maneuver is executed, the player now faces a long journey. Not
  wanting to wait, the player decides to do a faster-than-light jump.

NOTE: To give some context, a single orbit of the ISS around the earth takes
92.69 minutes at an altidude between 409 km and 416 km. To make the game not
suck, we'd have to compress that to a few minutes at most. Travelling from low
earth orbit to the moon takes about 5 days, according to my calculations. Even
compressed, that would still be hours without FTL.

- An FTL jump doesn't change anything about the course of the ship. The ship
  still ends up where it would end up without the FTL jump, it just does so
  instantly.
- The player activates the FTL drive by pressing the "FTL Jump" button. As the
  drive starts up, they select the target for the jump on the projected path by
  clicking where they want to end up. They choose the point along the path that
  is closest to the planet.
- With the target chosen, the player just needs to wait for the FTL drive to
  finish spinning up. Normally this would happen within a few seconds, but as
  the spin-up time rises with the mass of the ship, it takes a bit longer due to
  the full cargo hold.

OPEN QUESTION: How long does it take? This will have to be balanced with combat
mechanics, to make things as interesting as possible.

OPEN QUESTION: How much fuel does an FTL jump take? Maybe it shouldn't take a
lot, as the maneuver that brings the ship on course already costs fuel (and
costs more, the farther away the target is), and discouraging the use of FTL
could lead to boredom.

- Finally the drive has finished spinning up. The ship changes its location
  instantly, showing up directly near the planet. A moment later it reaches the
  point of the scheduled maneuver and breaks into orbit.
- Being in low orbit around the planet, the player can now access a menu that
  allows them to sell the metal they collected. They also decide to trade the
  metal for fuel and new modules for the ship.

NOTE: I'm being deliberately vague here, as the economy aspect deserves its own
scenario.

FUTURE EXTENSION: Eventually we'd want something better, like a player-driven
economy. For the beginning, we should keep it simple though, and being able to
exchange stuff at a planet seems like the best way to get the rest of the game
up and running first.


## Being Attacked By Pirates

- After having completed the New Player Experience (see previous scenario), the
  player decides to set out again in the hopes of finding more resources.
- Rather than heading back to the moon were they came from, the player decides
  to go to another moon this time. The player has heard that this moon has a
  large debris field in low orbit from a battle between player factions that
  happened a few weeks earlier.
- The player sets up the maneuvers for a Hohmann Transfer to the target moon.
  After a short wait, the ship arrives at the point of the first maneuver, which
  heightens the the opposite side of the ship's orbit, so the orbit intersects
  with the moon.
- After the ship has finished the maneuver, the player engages their FTL drive
  and selects the point on the predicated path that's closest to the moon as the
  target of the FTL jump. As the ship's cargo hold is empty this time, it only
  takes a few seconds for the FTL drive to charge.
- The ship arrives near the moon in an instant. As this is the point of the
  second maneuver, the ship starts executing the maneuver, which turns the
  ship's course into a low orbit around the moon.
- As the maneuver happens, sensor data starts coming in. The player's ship has
  landed near a small wreck. As this piece of debris is very close to the ship,
  its passive sensors are capable of recognizing it as a wreck, without the need
  to use active sensors.
- The player dispatches their drones to harvest the wreck. After a while, the
  wreck has been stripped on anything usable. The remains can no longer be
  picked up by sensors.
- The player navigates to a different wreck and harvests that, too. This repeats
  multiple times, with the player using active sensors to find new wrecks once
  in a while.
- While the player is working on a larger piece of wreck, the sensors pick up an
  unknown object that comes into view over the moon's horizon. The object is
  much faster than most other objects at this altitude, which indicates that it
  moves on a highly elliptical orbit, with the other side of the orbit much
  farther from the planet.
- The player pays no mind to this, as it is only one of many objects in orbit
  around the planet, most of which haven't been identified.
- As the object reaches its closest point to the player's ship, around 50 km
  inside the ship's orbit, the object starts emitting a large amount of energy
  (the heat of a thrusting engine) and starts decelerating. The player can
  recognize this from the text next to the object's symbol.
- As the ship's sensors pick this up, the object's classification changes to
  "ship". This is reflected by the symbol changing to the ship symbol.
- Almost immediately, the enemy ship starts firing missiles at the player's
  ship.
- The player's own missile launchers are set to defensive mode (the default
  setting), and start firing defensive missiles shortly after the enemy
  missile's launch. The defensive missiles intercept the enemy missiles and
  destroy them.
- The player decides to strike back and switches half of their missile launchers
  to offensive mode, selecting the enemy ship as a target. Instead of
  intercepting incoming missiles, the missiles launched from those launchers are
  programmed to fly directly to the enemy ship.
- After a short while, it becomes clear that the player is being overpowered.
  The enemy ship manages to destroy the player's missiles at about the halfway
  point between the ships, while the explosions from the enemy missiles come
  closer and closer.
- The player decides to flee. They execute a maneuver that brings them back to
  the safety of the planet and engage the FTL drive. As the cargo hold is almost
  full, the FTL drive takes a while to charge.
- As the enemy missiles manage to come closer and closer, it becomes clear that
  the FTL drive won't be charged in time.
- To prevent expensive damage to their ship, the player ejects all the collected
  cargo. This decreases the mass of the ship, reducing the energy that is
  required for an FTL jump. The energy requirement for the jump is fulfilled
  instantly, and the player's ship jumps away, leaving their hard-earned cargo
  behind for the pirate to collect.
