**Spoiler alert:** This repository contains solutions to first 19 days of [Advent of Code 2020](https://adventofcode.com/2020). If you plan to solve these problems yourself, you should probably not read the code.

On the day 20 I implemented the first part, trying to make it optimized, only to find out that for the second part would be easier if rewritten to remove the optimization, got annoyed and abandoned the thing. One day I might decide to finish it.

**Note:** code in this repo was written with the goal of solving the problem, not being read. I'm publishing it mostly to show my familiarity with Rust. When working with code that needs to be maintained I strive for a higher quality. Still, even now it should be quite readable.

Some highlights:
* in the day 4 problem solution I [box functions](src/day04b.rs), storing functional representation of validations in a vector,
* in the day 7 problem solution I use BFS (in [the first part](src/day07a.rs)) and DFS (in [the second part](src/day07b.rs)) to find paths in graph,
* in the solution for second part of day 13 problem I [implement](src/day13b.rs) simple module-inverse using extended Euclidean algorithm,
* in the solution for second part of day 16 problem I [implement](src/day16b.rs) maximum matching algorithm,
* in the day 17 problem solution I [define a structure to represent the 3-dimensional board and iterator to access neighbourhood of field](src/day17b.rs).
