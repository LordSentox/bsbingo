pub mod command;
mod load_words;

pub use command::*;
pub use load_words::*;

use std::io::{stdin, stdout, Write};
use std::{thread, time};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, terminal_size};

/// Clears the entire console, leaving an empty screen
pub fn console_clear() {
    print!("{}", termion::clear::All);
}

/// Shows everything that has been drawn to the console since the last clear
pub fn console_show() {
    stdout()
        .into_raw_mode()
        .expect("Failed to access stdout in raw mode")
        .flush()
        .expect("Failed to flush stdout");
}

fn main() {
    let (width, height) = terminal_size().expect("Could not get terminal size.");

    // Show a welcome splash
    console_clear();
    print!(
        "{}Hello there{}",
        termion::cursor::Goto(width / 2 - 5, height / 2),
        termion::cursor::Show
    );
    console_show();
    thread::sleep(time::Duration::from_millis(500));

    let mut command_line = CommandLine::new();

    console_clear();
    console_show();

    let stdin = stdin();
    for key in stdin.keys() {
        console_clear();

        if !command_line.handle_key(key.expect("Failed to read key from console")) {
            return;
        }

        //command_line.draw();

        console_show();
    }
}
