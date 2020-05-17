pub mod help;

use termion::event::Key;
use termion::{cursor, terminal_size};

use crate::{console_clear, console_show};

pub struct CommandLine {
    text: String,
    /// When the command line is in message mode, something is written in it,
    /// but it should not be interpreted as a command.
    message_mode: bool
}

impl CommandLine {
    pub fn new() -> CommandLine {
        CommandLine {
            text: String::new(),
            message_mode: false
        }
    }

    pub fn handle_key(&mut self, key: Key) -> bool {
        if self.message_mode {
            self.message_mode = false;
            self.text.clear();
        }

        match key {
            Key::Char('\n') => self.process_command(),
            Key::Backspace => {
                self.text.pop();
                true
            }
            Key::Char(c) => {
                self.text.push(c);
                true
            }
            _ => true
        }
    }

    pub fn show_message(&mut self, message: &str) {
        self.text = message.into();
        self.message_mode = true;
    }

    /// Process the command provided. Returns `true` if the program should
    /// continue running or `false` if the program should be exited as soon
    /// as possible.
    fn process_command(&mut self) -> bool {
        let command_words: Vec<String> = self
            .text
            .split_whitespace()
            .into_iter()
            .map(|w| w.trim().into())
            .collect();

        // When receiving an empty command, do nothing and continue with the program.
        if command_words.len() == 0 {
            return true;
        }

        if command_words[0].starts_with(':') {
            // Handle a dedicated command as opposed to a bullshit word you wanted to enter.
            match command_words[0].to_lowercase().as_str() {
                ":help" => help::show_help(),
                ":redraw" => {
                    console_clear();
                    self.show_message("redrawing...");
                    self.draw();
                    console_show();
                }
                ":exit" => return false,
                _ => self.show_message("Unknown command. Type :help to get a list of commands")
            }
        }
        else {
            // Handle a bullshit word, try to tick it off the card.
            // TODO
        }

        self.draw();
        true
    }

    pub fn height(&self) -> u16 {
        let (width, _) = terminal_size().expect("Could not get terminal size.");

        (self.text.len() / width as usize) as u16
    }

    /// Draw the command line to the bottom of the screen
    pub fn draw(&self) {
        let (_, height) = terminal_size().expect("Could not get terminal size.");

        print!(
            "{}{}{}",
            cursor::Goto(1, height - self.height()),
            self.text,
            cursor::Show
        );
    }
}
