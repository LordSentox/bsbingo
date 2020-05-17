pub mod command;
mod load_words;

pub use command::*;
pub use load_words::*;

use std::io::{stdin, stdout, Stdout, Write};
use std::{thread, time};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, terminal_size};

/// Clears the entire console, leaving an empty screen
pub fn console_clear(stdout: &mut Stdout) {
    write!(stdout, "{}", termion::clear::All).expect("Failed to clear console");
}

/// Shows everything that has been drawn to the console since the last clear
pub fn console_show(stdout: &mut Stdout) { stdout.flush().expect("Failed to flush stdout"); }

fn main() {
    let (width, height) = terminal_size().expect("Could not get terminal size.");

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Show a welcome splash
    console_clear(&mut stdout);
    write!(
        &mut stdout,
        "{}Hello there{}",
        termion::cursor::Goto(width / 2 - 5, height / 2),
        termion::cursor::Show
    )
    .unwrap();
    console_show(&mut stdout);
    thread::sleep(time::Duration::from_millis(500));

    let mut command_line = CommandLine::new();

    console_clear(&mut stdout);
    console_show(&mut stdout);

    for key in stdin.keys() {
        console_clear(&mut stdout);

        if !command_line.handle_key(key.expect("Failed to read key from console"), &mut stdout) {
            return;
        }

        command_line.draw(&mut stdout);

        console_show(&mut stdout);
    }
}
