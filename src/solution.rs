use crate::agents::Agent;
use crate::board::{Board, Cell};
use crate::player::Player;

// Your solution agent.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your agent will make.
    fn solve(board: &mut Board, player: Player) -> (i32, usize, usize) {
        // If you want to make a recursive call to this agent, use
        // `SolutionAgent::solve(...)`
        todo!("Not implemented yet!");
    }
}
