mod board;
mod player;
mod agents;
mod manual;
mod solution;
mod args;

use board::{Board, Layout, Cell};
use player::Player;
use agents::{Agent, FirstMoveAgent, RandomAgent, TestAgent};
use manual::ManualAgent;
use solution::SolutionAgent;
use args::{Args, Agents};

use clap::Parser;
use std::time::{Duration, SystemTime};
use std::{thread, process};

const TIME_LIMIT: u64 = 1500;
const WAKE_UP_COUNT: u64 = 100;


// Invoke the solver.
fn invoked_solver(args: Args, mut board: Board, player: Player) -> (f32, usize, usize) {
  let agent = args.get_agent(player);

  let handler = thread::spawn(move || {
    let timer = SystemTime::now();

    let (_, row, col) = match agent {
      Agents::First => FirstMoveAgent::solve(&mut board, player),
      Agents::Random => RandomAgent::solve(&mut board, player),
      Agents::Test => TestAgent::solve(&mut board, player),
      Agents::Solution => SolutionAgent::solve(&mut board, player),
      Agents::Manual => ManualAgent::solve(&mut board, player),
    };

    let elapsed = timer.elapsed().unwrap();
    let time_in_seconds = (elapsed.as_millis() as f32) / 1000.0;

    return (time_in_seconds, row, col);
  });

  // Manual agent does not have a time limit.
  if let Agents::Manual = agent {
    return handler.join().unwrap();
  }

  // Sleep for 1 second, periodically waking up to check if agent is done.
  // If it was never done by the end, kill it.
  let sleep_time = Duration::from_millis(TIME_LIMIT / WAKE_UP_COUNT);
  for i in 0..WAKE_UP_COUNT {
    thread::sleep(sleep_time);

    if handler.is_finished() {
      return handler.join().unwrap();
    }
  }
  
  println!("{} Agent is taking too long. Game forfeited", player.to_string());
  process::exit(1);
}


// The main function.
fn main() {
  // Parse arguments.
  let args = Args::parse();

  let mut board = Board::new(args.get_layout());
  println!("Game begins");
  board.print();

  let mut max_x_time: f32 = 0.0;
  let mut max_o_time: f32 = 0.0;
  
  let mut player = Player::X;
  while !board.game_over() {
    let (time, row, col) = invoked_solver(args.clone(), board.clone(), player);
  
    board.apply_move((row, col), player);
    board.print();

    println!("Agent took {:.2} seconds to move", time);
    match player {
      Player::X => if time > max_x_time {
        max_x_time = time;
      },
      Player::O => if time > max_o_time {
        max_o_time = time;
      }
    }

    player = player.invert();
  }
  
  let score = board.score();
  if score > 0 {
    println!("X wins. Final score {}", score);
  } else if score < 0 {
    println!("O wins. Final score {}", score);
  } else {
    println!("Draw");
  }
  
  println!("X's slowest move took {} seconds", max_x_time);
  println!("O's slowest move took {} seconds", max_o_time);
}
