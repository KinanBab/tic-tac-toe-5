// No-op fake colors for printing to terminal in Windows
// Termion does not support windows.
pub enum Color {
    Yellow,
    Green,
    Red,
    Blue,
    Reset,
}

pub use Color::*;

pub fn Fg(_: Color) -> &'static str {
  ""
}
