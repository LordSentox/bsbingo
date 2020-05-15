mod load_words;
pub use load_words::*;

use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, terminal_size};

fn main() {
    let (width, height) = terminal_size().expect("Could not get terminal size.");

    write!(
        stdout(),
        "{}{} Hello there {}",
        termion::clear::All,
        termion::cursor::Goto(width / 2 - 5, height / 2),
        termion::cursor::Show
    )
    .unwrap();
    stdout().into_raw_mode().unwrap().flush().unwrap();

    for _ in stdin().keys() {
        break;
    }
}
