# 5x5 Tic Tac Toe with Walls

This repo contains a stencil for a 5x5 tic tac toe game with random walls.

The purpose of the game is to get more consecutive sequences X's or O's than
your opponent. Each 3 consecutive X's increase the X player score by 1, and similarly
for the O player. The player with the higher score wins. The sequence can be over
a row, column, or diagonal.

## Install and build the code

Download the contents of this repo, and store them in a new folder on your computer.

After downloading the repo, open a terminal in its directory and run the following command to compile the code.
```bash
cargo b
```

### Step 1: Run two random agents

Have our two random agents play against each other. Run the following command and observe the output.
```bash
cargo run -- --x random --o random --layout 5
```

### Step 2: Play a game against a random agent!

Use the following command to play against the random agent, you will be the X player.
```bash
cargo run -- -x manual --o random --layout 5
```

### Step 3: Explore

In general, you can run the stencil code with different agents and for different board layouts!
```
cargo run -- --x <agent> --o <agent> --layout <layout>
```

**agent** can be any of `first`, `random`, `test`, `manual`, and `solution`.

**layout** can either be `3x3`, or a number between `0` and `10`. The first is a premade
layout that is identical to a regular 3x3 tic tac toe. The later correspond to a layout
with as many walls as the provided number, which are randomly placed over the board.


For example, the following command use the test agent for X, the random agent for O, on a board with 5 random walls.
```
cargo run -- --x test --o random --layout 5
```

## Implementing your solution

You can open the **folder** where you stored the content of this repo using VSCode to implement your solution.

You can implement your solution insde `src/solution.rs`. **DO NOT MODIFY ANY OTHER FILES**

Your solution must select the next move within 1.5 seconds.

If it takes too long, your agent will be killed and you forfeit the game to the other player.

We suggest you follow the implementation plan below. This is designed to ensure that everyone is able
to meet the minimum requirements for this project.

**We will ask you to fill a form each day to keep track of your progress via canvas.**

### Implementation plan

1. Tuesday July 22nd: Implement Min max for this game inside the Solution agent in `src/solution.rs`.

Use your favorite implementation of Min max from homework 7, or your own implementation for Leetcode's divisor game from class, as a reference. I personally like the [one function and enum version](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=49ca9fa2e5f17536dbe3af959d06584c).

You can also use my reference [3 by 3 tic tac toe implementation](https://github.com/KinanBab/tic-tac-toe) as a reference if you are completely stuck.

Make use of the provided functions in `src/board.rs`. Specifically, you can use `board.score()` to get the current score, `board.moves()` to get all the available valid moves, and `board.apply_move(<move>, <player>)` to apply the move by the player to the board (in place).

Remember that min max explores all possibilities. It will exceed the assigned time limit on this game's 5x5 board.

To be able to test your implementation, we provide you with a preset layout named `3x3`. This layout has walls on all the outer rows
and columns, thus it results in only a 3x3 usable area of the board. On this preset, your min max implementation should execute within the time limit.

```
This is what the 3x3 board looks like initially:

W | W | W | W | W
W |   |   |   | W
W |   |   |   | W
W |   |   |   | W
W | W | W | W | W
```

To run your agent on this layout, use the following command:
```
cargo run -- --x solution --o first --layout 3x3
```

Hint 1: It is ok if your first implementation is slow. Focus on making sure it works correctly!
Hint 2: Try to avoid un-necessary clones to make sure your implementation is as fast as possible. The faster it is, the more possibilities you'll be able to explore within the time limit!
Hint 3: When your agent makes a move during min max, consider undoing that move after it is no longer needed and before applying other independent moves. This will allow you to not copy/clone your board.
Hint 4: Look at `src/board.rs` to find functions that will allow you to apply and undo a move.

2. Wednesday July 23rd: Add a heuristic and limit the number of possibilities Min max explores.

Modify your implementation to only explore possibilities up to a fixed depth. Try to set the depth to be the biggest possible value with which your implementation still succeeds within the time limit.

You should be able to explore 4 levels deep (i.e. two moves by X and two moves by O) and still remain within the time limit. You may be able to do more levels if your implementation is more optimized. However, 4 should be sufficient to meet the requirements of this project.

After exhausting the number of levels, your modified implementation needs to assign the board a score, even when it is not game over. To do that we use a heuristic.

Add a heuristic function to `src/solution.rs` to assign scores to incomplete boards.

Hint 1: The heuristic function signature should be `fn heruistic(board: &Board) -> i32`.
Hint 2: `board.score()` is a good starting heuristic. Try to understand what that function does.

You should test your modified agent on the full board, not the 3x3 layout. You can do that using this command:
```
cargo run -- --x solution --o first --layout 5
```

3. Thursday July 24th: Experiment to ensure your agent is intelligent enough, and improve your heuristic if needed.

There are many many ideas for how you can make your agent more intelligent. You could make the heuristic better, find ways to explore more than 4 levels deep, or something else completely! Be creative!

Run your agent against the various agent we provide you: `first`, `random`, and `test`.

Play against each of these agents at least 10 times with `--layout 5`.

**The minimum requirement for completing this project is for your agent to beat `first` and `random` almost always, and to beat `test` over half the times, when your agent is the X player.**

Ensure that you're agent is functional when it's used for the O player as well. We will use that in the tournament (more on this later).

Feel free to discuss with your other classmates, search the internet, ask chatGPT, or talk to the teaching staff to find ideas to improve the agent if need be!

4. Friday July 25th: In class tournament.

**VERY IMPORTANT: Submit your `solution.rs` file via email to `kinan_dak_albab@brown.edu` by Thursday 11:59PM.**

We will organize a tournament in class on Friday. The tournament will be a single elimination bracket style.
In each round, we will run your code against another group's implementation 4 times. Twice as X and twice as O.
The first agent to win 3 games proceeds to the next round.
In case of a tie, we will play a 1 game decider and use a coin toss to determine who is X and who is O.

The winners will receive a prize!




