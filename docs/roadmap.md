# Roadmap

## Overview

This document tracks the short-term goals. For a more long-term view, refer to
docs/vision.md.


## Future Milestones

### First Prototype

A first prototype that shows a glimpse of what the game could look like. Once
this milestone is reached, I want to start talking about VNDF more publically,
actively use marketing instruments such as a mailing list and start looking into
early monetization strategies.

Features:

* **A large planet**: This can be rendered simply as a circle. For the planet to
  be presented properly, we need camera zoom.
* **Orbital mechanics**: Any objects beside the planet (player ships, missiles)
  are subject to orbital mechanics. The player ship starts in a circular orbit
  around the planet.
* **Path visualization**: Object's orbits are visualized as ellipses (similar to
  Kerbal Space Program). I already have working code that does this in other
  projects (Tiny World War, Orbital Invaders).
* **Maneuvers**: Player ships can maneuver using a maneuver node system similar
  to Kerbal Space Program. This can be really simplistic (delta-v being applied
  instantly at the point of the maneuver).
* **Collision Detection**: Collisions between ships/missiles and the planet
  destroy the ship/the missile. Collisions between ships and missiles destroy
  the missile.
* **Die/Respawn**: After a ship is destroyed, it disappears. The player still
  receives updates. The player can respawn by pressing a button.
* **Targets**: Players can select other ships as targets. There always is one
  active target.
* **Missiles**: A ship continuously shoots missiles at the selected target.
  Missiles accelerate towards the target and explode near it.

No real gameplay yet, but some mechanics to play around with. I strongly suspect
I'll want to re-introduce 3D after seeing this play out, but who knows. The
results of this prototype will inform further decisions.
