use crate::player::Player;

use termion::color;

// The contents of a single cell.
#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    X,
    O,
    Empty,
    Wall,
}

impl Cell {
    pub fn to_string(&self) -> String {
        match self {
            Cell::X => format!("{}X{}", color::Fg(color::Green), color::Fg(color::Reset)),
            Cell::O => format!("{}O{}", color::Fg(color::Blue), color::Fg(color::Reset)),
            Cell::Empty => String::from(" "),
            Cell::Wall => format!("{}W{}", color::Fg(color::Red), color::Fg(color::Reset)),
        }
    }
}

// Presaved board layouts.
pub enum Layout {
    ThreeByThree,
    Empty,
    Random(usize),
}

// The board game state.
#[derive(Clone)]
pub struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    // Create a new board.
    pub fn new(layout: Layout) -> Board {
        let board_cells = match layout {
            // Walls all around, three by three empty cells in the middle.
            Layout::ThreeByThree => vec![
                vec![Cell::Wall; 5],
                vec![
                    Cell::Wall,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Wall,
                ],
                vec![
                    Cell::Wall,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Wall,
                ],
                vec![
                    Cell::Wall,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Wall,
                ],
                vec![Cell::Wall; 5],
            ],

            // No walls.
            Layout::Empty => vec![vec![Cell::Empty; 5]; 5],

            // Random walls, the count of walls is determined by `walls`.
            Layout::Random(walls) => {
                use rand::Rng;

                let mut locations = std::collections::HashSet::new();
                while locations.len() < walls {
                    let i: usize = rand::thread_rng().gen_range(0..5);
                    let j: usize = rand::thread_rng().gen_range(0..5);
                    locations.insert((i, j));
                }
                let mut cells: Vec<Vec<Cell>> = vec![vec![Cell::Empty; 5]; 5];
                for (i, j) in locations {
                    cells[i][j] = Cell::Wall;
                }
                cells
            }
        };

        return Board { cells: board_cells };
    }

    // Return all legal moves available on this board.
    pub fn moves(&self) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                if let Cell::Empty = self.cells[i][j] {
                    moves.push((i, j));
                }
            }
        }
        return moves;
    }

    // Check if the game is over.
    pub fn game_over(&self) -> bool {
        return self.moves().len() == 0;
    }

    // Score the game. This counts the number of 3 consecutive Xs and 3 consecutive Os,
    // and returns their difference.
    // If score > 0, then there are more consecutive Xs than Os, and the X player wins.
    // If score < 0, then the O player wins.
    // Score == 0 indicates a draw.
    pub fn score(&self) -> i32 {
        let mut score: i32 = 0;
        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                // Count row.
                if j + 2 < self.cells.len() {
                    let x = &self.cells[i][j];
                    let y = &self.cells[i][j + 1];
                    let z = &self.cells[i][j + 2];
                    if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
                        score += 1;
                    } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
                        score -= 1;
                    }
                }
                // Count col.
                if i + 2 < self.cells.len() {
                    let x = &self.cells[i][j];
                    let y = &self.cells[i + 1][j];
                    let z = &self.cells[i + 2][j];
                    if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
                        score += 1;
                    } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
                        score -= 1;
                    }
                }
                // 1st diagonal
                if i + 2 < self.cells.len() && j + 2 < self.cells.len() {
                    let x = &self.cells[i][j];
                    let y = &self.cells[i + 1][j + 1];
                    let z = &self.cells[i + 2][j + 2];
                    if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
                        score += 1;
                    } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
                        score -= 1;
                    }
                }

                // 2nd diagonal
                if i + 2 < self.cells.len() && j >= 2 {
                    let x = &self.cells[i][j];
                    let y = &self.cells[i + 1][j - 1];
                    let z = &self.cells[i + 2][j - 2];
                    if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
                        score += 1;
                    } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
                        score -= 1;
                    }
                }
            }
        }

        return score;
    }

    // Apply the given move to the board.
    // This modifies the board, it does not copy it.
    // To ensure independent moves do not interfer with each other in your min max
    // implementation either:
    //
    // (1) copy the board before applying a move, e.g.
    //        let board2 = board.clone();
    //        board2.apply_move(m, player);
    //
    // (2) undo the move after it is no longer needed, e.g.
    //        board.apply_move(m, player);
    //        ... code that uses board here ...
    //        board.undo_move(m, player);
    //     You must undo the move before applying any other *independent* moves
    //     to the board.
    //     The value of player should be the same when applying and undoing a move.
    pub fn apply_move(&mut self, m: (usize, usize), player: Player) {
        if let Cell::Empty = self.cells[m.0][m.1] {
            match player {
                Player::X => self.cells[m.0][m.1] = Cell::X,
                Player::O => self.cells[m.0][m.1] = Cell::O,
            };
        } else {
            panic!("Illegal move");
        }
    }

    // Undos the given move by the given player.
    // Sets the location of the cell to empty, if it's content matches the player.
    pub fn undo_move(&mut self, m: (usize, usize), player: Player) {
        match (player, &self.cells[m.0][m.1]) {
            (Player::X, Cell::X) => self.cells[m.0][m.1] = Cell::Empty,
            (Player::O, Cell::O) => self.cells[m.0][m.1] = Cell::Empty,
            _ => panic!("Illegal undo move"),
        }
    }

    // Print board to the screen.
    pub fn print(&self) {
        println!("-----------");
        for row in &self.cells {
            println!(
                "  {} | {} | {} | {} | {}",
                row[0].to_string(),
                row[1].to_string(),
                row[2].to_string(),
                row[3].to_string(),
                row[4].to_string()
            );
        }
        println!("-----------");
    }

    // Returns a read-only reference to the underlying 2D vec.
    // You may use this in your heuristic to look at the values of specific cells,
    // or loop over the entire board, etc.
    pub fn get_cells(&self) -> &Vec<Vec<Cell>> {
        return &self.cells;
    }
}
