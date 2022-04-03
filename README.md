# poorguelike (temporary name, accepting ideas)

![game gif](./game.gif)

This is my first (serious) attempt at both:
- Making a game
- Using Rust

I'm trying to use the least amount of dependencies possible, so for now the only lib I'm depending on is [raylib](https://www.raylib.com/) via [raylib-rs](https://github.com/deltaphc/raylib-rs).

For art is using this great [Kenney](kenney.nl) tileset (I'm not an artist, but I'll probably expand it further if I need more stuff): https://www.kenney.nl/assets/bit-pack

## Motivation

I get bored out of my mind by practicing leetcode/learning algorithm/data structure implementations and advanced theory, so I figured why not try making something a little bit more difficult by applying those concepts to something more tangible that I would actually enjoy coding? So here we are.

I didn't really want to use a big engine or anything since that would hide the complicated things in ***magic abstractions*** (although one could argue raylib hides the "drawing pixels/textures to a framebuffer" part, but I'm fine with that part being hidden - at the time I'm writing this it felt very hard to learn the intricacies of drawing pixels to the screen, so I just chose to use something to make that part easier) and I figured, since I want it to be as hard as possible (I will eventualy need to learn how to use those things I need to learn) let's try to do it as *"vanilla"* as possible.

Also I've been putting off learning Rust for a while, so why not tackle that alongside learning how to make a game? :^)

I chose to do a roguelike because 3d feels very hard to grasp right now, so I'll use this project as a stepping stone to learn 3d game building next!

## Idea

I really like the following ideas of roguelikes:
- A new game (almost) never looks like the previous one
- Permadeath
- Complex systems of:
  - Combat
  - Allies
  - Items
  - There are a lot of complex systems on cool roguelikes!
- Simple graphics (I chose to use tiles here because *ascii* although charming (for me) is not very approachable if I want to show this to my non-programmer friends that might not have played roguelikes before
