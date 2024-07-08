use crate::board::{Board, Cell};
use crate::player::Player;
use crate::agents::Agent;

// Your solution agent.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
  fn solve(board: &mut Board, player: Player) -> (i32, usize, usize) {
    // If you want to make a recursive call to this agent, use
    // `SolutionAgent::solve(...)`
    todo!("Not implemented yet!");
  }
}
