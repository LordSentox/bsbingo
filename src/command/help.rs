use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn show_help() {
    let help = ":help -- Show this help message\n
                :exit -- Quit the program\n
                :redraw -- Render information on the screen again, especially useful when the program bugs on you which never happens, of course\n
                Entering something without using a leading : will be interpreted as entering a word that you have heard and want to mark.\n";
    print!("{}{}", termion::clear::All, help);

    stdout().into_raw_mode().unwrap().flush().unwrap();

    // Wait for a key to be pressed to return to the main program
    for _ in stdin().keys() {
        break;
    }
}
