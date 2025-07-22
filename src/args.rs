// This handles reading the command line arguments for configuring the board layout
// and which agents to use for each player.
use clap::{Parser, ValueEnum};

use crate::board::Layout;
use crate::player::Player;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    // X player agent
    #[arg(short, long, value_enum)]
    pub x: Agents,

    // Y player agent
    #[arg(short, long, value_enum)]
    pub o: Agents,

    // Layout
    #[arg(short, long)]
    layout: String,
}

impl Args {
    pub fn get_layout(&self) -> Layout {
        if self.layout == "3x3" {
            return Layout::ThreeByThree;
        } else if self.layout == "0" {
            return Layout::Empty;
        } else {
            return Layout::Random(self.layout.parse().unwrap());
        }
    }

    pub fn get_agent(&self, player: Player) -> Agents {
        match player {
            Player::X => self.x,
            Player::O => self.o,
        }
    }
}

// The agent possibilities we have.
#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Agents {
    First,
    Random,
    Test,
    Solution,
    Manual,
}
