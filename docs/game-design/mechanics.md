# Mechanics

This document describes ideas for game mechanics for VNDF. For the longest time,
I didn't write down most of my ideas. On the one hand, because there was noone
that needed to read them, but also because I realize that all ideas suck, and
need to change before they don't.

Unproven ideas are just speculation. Some might work as imagened. More likely,
an idea won't work the way it was imagined, and will need to be adapted. In
almost all cases, experimenting with an implementation will lead to something
better than was originally imagined. For this reason, I won't go into a lot of
detail in most cases.

However, now we're a team, and we need some basis to talk about. I'll attempt to
write down the ideas that I have in mind and want to try out in the future, so
others (well, Chris) can give their feedback and don't have to fly blind until,
except for the small chunks of wisdom I drop here and there.

Please take everything in this document for what it is: A collection of concepts
that might or might not make it into the game, and will be changed and adopted
along the way. Please feel free to open discussions about anything on Trello, or
add your own ideas to this document.


## Overarching Game Structure

Note: Please read the
[back story](https://github.com/hannobraun/vndf/blob/master/docs/game-design/story.md)
before reading this section.

Core goal number 5 requires a sense of adventure and danger, with forward
movement being forced upon the player by the circumstances. My motivation for
adding that goal was that I see VNDF as a sandbox game[1], but I don't like the
way most sandbox games play out after a while. It starts out with a lot of fun,
as you discover the world and fight against its dangers, but at some point you
have all the basics covered, advancing further has become a lot of work, and
even if you are in a dangerous situation, it's no longer exciting but just
frustrating, as dying usually causes you more mindless to replace your losses.

On the other hand, I really admire what FTL does. It puts the player into a kind
of sandbox, but it doesn't let the player move freely in that sandbox. The
player's decisions are very constrained by the resources they are given, and
they are being hunted by an enemy that they have no chance of winning against.
This makes every decision count in a way that I've never seen a "real" sandbox
game do beyond the first phase of gameplay.

Now, FTL is a single-player game, so it has a lot of freedom in adapting its
world to reach any goals it likes. Unless we turn VNDF into some kind of fake
multiplayer game, where every player lives in their own reality, we can't do
that[2].

I've thought long and hard for a solution to this problem, and I think I've come
up a solution for this problem: Players live in the outer solar system, where
they lack cheap solar energy and thus the basis for wealth and industry. To
improve their lot, they are forced to raid the inner solar system, stealing fuel
and supplies from the von Neumann probes.

The efficiency of propulsion drives in the game would be balanced, so that the
trip there would require most of a ship's fuel. Once in the inner solar system,
beyond stealing for riches, players are forced to find fuel to be able to make
it back at all. Meanwhile, they are hunted by the probes, being forced to stay
on the move while scavenging, performing surprise attacks on lightly defended
installations etc.

This allows us to turn each raid into an exciting adventure in the same way that
FTL does with its singleplayer campaign.

Once the player makes it back, they can repair their ship, sell their loot and
plan the next campaign. I'd imagine that initially the game play in the outer
solar system would be very minimal (sell ore and fuel, buy bigger ship), but
later we could support a full player economy out there.

[1] I don't like MMOs that are overly structured, as I believe they throw away
    most of the potential that an MMO gives you in the first place.
[2] See Guild Wars as an example of an MMO that does this. Each player starts in
    a lush, green fantasy kingdom. Then an apocalyptic event happens and the
    kingdom becomes devastated. Meanwhile, other players keep running around on
    the lush green map, because the apocalypse hasn't happened for them, yet.


## Realistic Orbital Mechanics

I'm a big fan of orbital mechanics. They are quite unintuitive at first, but
once you get it, it's a lot of fun, and they add depth to the game. They also
allow support the core goal of making player skill a prime motivation for
success.

Player ships should basically be bound by the same constraints that our
contemporary space probes are, but with a slightly higher level of technology
(meaning more efficient propulsion).


## FTL Travel

I don't think a game within a solar system of semi-realistic proportions would
work without some form of faster-than-light travel. I like the Battlestar
Galactica style of FTL, where you just disappear in one spot and appear in
another, both regarding to general style and to gameplay (why should players
wait around, being bored while their ship warps somewhere).

However, that opens up a few questions, mostly, what happens to the velocity
when you jump. From the perspective of orbital mechanics, this is very
important. A velocity that is high enough to keep you in orbit around a small
moon will be too low for a gas giant.

I have come up with the following model:

- From a physics perspective, we model this as local time acceleration.
- This means, the ship flies on the same course it would have taken, if it had
  just coasted along for months. Only, it shows up at the destination instantly.
- That means it takes some skill to navigate to another planet (and if you don't
  believe me that this is a lot of fun, then I dare you to play Kerbal Space
  Program for a while).

Bonus idea:

- The FTL drive takes some time to spool up. This time scales with the mass of
  the ship (maybe proportional, or something more extreme like quadratic).
- That means, if you're suprise-attacked, you're incentivized to dump your cargo
  in order to save yourself.
- This enables inter-player piracy that is profitable for the pirate without
  being completely devastating to the victim (you just lose some of your cargo).


## Space Ship Customization

I'd like to give players a lot of freedom when customizing their space ships.
Instead of taking the EVE Online route of "pre-designed ships + module slots",
I'd like to make ships just be a collection of modules, in a lego-like fashion.

There would be some basic modules that every ship has:

- Armor
- Cargo hold
- Propulsion system (maybe different ones with different trade-offs, i.e.
  chemical engines and ion engines).
- Fuel tanks

As well as more specialized modules:

- Weapons
- Industrial facilities, like refineries (ore -> metal), workships (metal ->
  module) etc.
- Repair/customization facilities, allowing you to repair or customize
  (add/remove modules) your ship or neighbouring ones

For example, you could build a lightly-armed all-round ship for all situations,
a very specialized battleship, or you build one out of just cargo holds and
repair facilities and you have a shipyard.

Initially, the modules would just be represented in a list (this ship has 1 unit
of fuel tank and 2 units of engines), but later, we can lay them out on a 2D
grid in a Lego-like fashion. Together with a basic damage model, that would add
more depth to ship design (for example, a player could put all amor on the front
of an attack craft to save weight, with all the trade-offs that implies).

Each module would have attributes, like:

- Mass
- Armor
- Cargo capacity (would be 0 for anything that's not a cargo hold)
- Fuel capacity (0 for anything that's not a tank)
- Thrust (0 for anything that's not an engine)

The complete ship would have the same attributes, which are simply a sum of all
its modules.


## Combat

I prefer combat to be based on missiles, because it reinforces the "low-tech"
scenario and because missiles have mass and require space to be stored, which
means lots of trade-offs for the player.

I've come up with the following model mostly by thinking things through
logically. I believe the result is both believable and interesting from a game
design perspective.

About missiles:

- Missiles can be large, small, or anything in between.
- Large missiles can have more fuel, which means more range.
- But large missiles are ineffective at short range, simply because they need
  time to accelerate, and a slow missile is easy to shoot down.
- Small missiles have low mass and can be accelerated quickly, making them
  effective at short range.
- Because they can't store a lot of fuel, small missiles have short range.
- We could give players lots of freedom in designing their missiles, similar to
  ships. That means players could decide how much explosives and fuel to put in,
  thereby influencing mass, range and damage potential.
- Missiles simply explode as close to the target as possible, damaging the
  target via the high-speed debris they produce.

Launching missiles:

- Ships would have small launchers to shoot small missiles, large launchers for
  large missiles etc.
- Launchers differ from each other not only in size, but whether they just drop
  the missile and let it accelerate itself or whether they add some initial
  acceleration themselves.
- Accelerating launchers give missiles initial relative velocity (higher range,
  more effective at short range), but require a lot of power to operate.
- Depending on how much fuel a player is willing to spend on a fight, they can
  be more effective or less, making logistics very important.

Combat in general:

- I don't like the concept of regenerating shields. I think it's much more
  interesting, if ships have to avoid hits by shooting down the incoming
  missiles with smaller missiles.
- A direct hit would cause damage that is hard and costly to repair (requires
  spare parts, raw material). Your "shields" are your defensive missiles, that
  keep enemy missiles away from you.
- If you don't have enough defensive capability, the explosions will come closer
  and closer, until they are close enough to cause damage.
- The player can decide on their strategy: Devote more missile launchers to
  defense or go all-out offensive?
- Of course you would be limited by the kind of missiles you have. If you only
  have the high-explosive large missiles with you, you can't do much defense.
- Defensive missiles are overall more flexible than shields would be. Not only
  can you weigh between offense and defense, you could also decide to use your
  defensive capability for another nearby ship (make sure the loot gets away).
