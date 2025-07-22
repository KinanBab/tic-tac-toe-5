use crate::board::Board;
use crate::player::Player;

pub trait Agent {
    // Must return (score, x coordinate of move, y coordinate of move).
    fn solve(board: &mut Board, player: Player) -> (i32, usize, usize);
}

// A dumb agent that makes the first possible move.
pub struct FirstMoveAgent {}
impl Agent for FirstMoveAgent {
    fn solve(board: &mut Board, _player: Player) -> (i32, usize, usize) {
        let moves = board.moves();
        return (0, moves[0].0, moves[0].1);
    }
}

// A dumb agent that makes moves randomly.
pub struct RandomAgent {}
impl Agent for RandomAgent {
    fn solve(board: &mut Board, _player: Player) -> (i32, usize, usize) {
        use rand::Rng;

        let moves = board.moves();
        let i: usize = rand::thread_rng().gen_range(0..moves.len());
        return (0, moves[i].0, moves[i].1);
    }
}

// Slightly better agent based on a heuristic but without any min-max.
pub struct TestAgent {}
impl Agent for TestAgent {
    fn solve(board: &mut Board, player: Player) -> (i32, usize, usize) {
        let moves = board.moves();

        // Assume first move is best move for now.
        let mut best_move = moves[0];
        board.apply_move(best_move, player);
        let mut best_score = board.score();
        board.undo_move(best_move, player);

        // Look at the remaining moves, see if any seem better.
        for i in 1..moves.len() {
            let m = moves[i];
            board.apply_move(m, player);
            let score = board.score();
            board.undo_move(m, player);

            match player {
                Player::X => {
                    if score > best_score {
                        best_move = m;
                        best_score = score;
                    }
                }
                Player::O => {
                    if score < best_score {
                        best_move = m;
                        best_score = score;
                    }
                }
            }
        }
        return (best_score, best_move.0, best_move.1);
    }
}
