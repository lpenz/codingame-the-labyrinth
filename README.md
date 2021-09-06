[![CI](https://github.com/lpenz/codingame-the-labyrinth/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/codingame-the-labyrinth/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/codingame-the-labyrinth/badge.svg?branch=main)](https://coveralls.io/github/lpenz/codingame-the-labyrinth?branch=main)


# codingame-the-labyrinth

This repository has my solution for [The Labyrinth] exercise of
[Codingame], made in rust.


## Overall strategy

This is a grid-based exercise, where we have to find and go over a
target coordinate, and then go back to the start of the map as fast as
possible.

The overall strategy is pretty simple, and can be divided in the
following phases:
- *discovery*: explore the "unknown" coordinates of the map, using a
  path-finding algorithm to move towards the closest "unkown"
  tile. Active until we find a path to the "control room".
- *activation*: after we find the control room, we use path-finding to
  move onto it.
- *escape*: use path-finding to get the optimal path to the starting
  position.


This repository just uses BFS for all path-finding. I was expecting to
hit a situation where the *discovery* phase would not explore enough
for us to find an optimal path for the *escape* phase, but that didn't
happen.


## File structure

This repository follows my usual file structure for codingame
challenges (both puzzles and AI):
- [src/core.rs](src/core.rs): the core datastructures and
  functionality.
- [src/input.rs](src/input.rs): input parsing, inclusing the `FromStr`
  trait implementations.
- [src/error.rs](src/error.rs): error enum. We can just `unwrap()`
  everything, but I'd rather exercise the best practices.
- [src/entrypoint.rs](src/entrypoint.rs): the main loop, where the
  strategy is implemented.
- [src/sqrid.rs](src/sqrid.rs): this file is imported from the
  [sqrid](https://github.com/lpenz/sqrid) square-grid library. It has
  the implementation of all the grid-related functions, in an
  easy-to-import format. I'm using this in other square-grid related
  puzzles and AI bots.


[Codingame]: https://www.codingame.com/
[The Labyrinth]: https://www.codingame.com/ide/puzzle/the-labyrinth
