use crate::agents::Agent;
use crate::board::{Board, Cell};
use crate::player::Player;

use termion::color;

// Agent that just prompts users to play the game via the terminal.
pub struct ManualAgent {}
impl Agent for ManualAgent {
    fn solve(board: &mut Board, _player: Player) -> (i32, usize, usize) {
        let cells = board.get_cells();

        println!("");
        println!("");
        println!("");
        println!("");
        println!("Enter your move below by typing in the number shown in the desired cell:");
        println!("");

        // Print the board with the possible move numbers to the terminal.
        let mut counter = 0;
        for i in 0..cells.len() {
            for j in 0..cells.len() {
                match &cells[i][j] {
                    Cell::Empty => {
                        print!(
                            " {}{:02}{} |",
                            color::Fg(color::Yellow),
                            counter,
                            color::Fg(color::Reset)
                        );
                        counter += 1;
                    }
                    _ => print!("  {} |", cells[i][j].to_string()),
                }
            }
            println!("");
        }

        // Read the input number from the terminal.
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let input: usize = line.trim().parse().expect("Input not an integer");

        // Make the move.
        let m = board.moves()[input];

        println!("");
        println!("");
        println!("");

        return (0, m.0, m.1);
    }
}
