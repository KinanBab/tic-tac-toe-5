# 5x5 Tic Tac Toe with Walls

This repo contains a stencil for a 5x5 tic tac toe game with random walls.

The purpose of the game is to get more consecutive sequences X's or O's than
your opponent. Each 3 consecutive X's increase the X player score by 1, and similarly
for the O player. The player with the higher score wins. The sequence can be over
a row, column, or diagonal.

## Installation instructions

Download the contents of this repo, and store them in a new folder on your computer.



After downloading the repo, open a terminal in its directory and run the following command to compile the code.
```
cargo b
```

To run the stencil code, use the following command.
```
cargo run -- --x <x player agent> --o <o player agent> --layout <wall layout>
```

The available agents are `first`, `random`, `test`, `manual`, and `solution`.

Layout can either be `3x3`, or a number between `0` and `10`. The first is a premade
layout that is identical to a regular 3x3 tic tac toe. The later correspond to a layout
with as many walls as the provided number, which are randomly placed over the board.


For example, the following command use the test agent for X, the random agent for O, on a board with 5 random walls.
```
cargo run -- --x test --o random --layout 5
```

## Implementing your solution

You can open the folder where you stored the content of this repo using VSCode or a similar Rust IDE to implement your solution.

You can implement your solution insde `src/solution.rs`. **DO NOT MODIFY ANY OTHER FILES**

Your solution must select the next move within 1.5 seconds.

If it takes too long, your agent will be killed and you forfeit the game to the other player.

## Implementation plan

Coming soon.
