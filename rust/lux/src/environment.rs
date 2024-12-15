use std::io::{prelude::*, stdin, StdinLock};
use super::*;

/// Represents Action performed by Agent
pub type Action = String;


/// Environment wrapper to interact with Lux AI API I/O
pub struct Environment {
    actions: Vec<Action>,
    curr_line: String,
    stdin: StdinLock<'static>,
}

impl Environment {
    /// Initializes Environment with stdout stdin
    ///
    /// # Parameters
    ///
    /// None
    ///
    /// # Returns
    ///
    /// A new created `Environment`
    pub fn new() -> Self {
        Self {
            actions: vec![],
            curr_line: String::new(),
            stdin: stdin().lock(),
        }
    }

    pub fn read_line(&mut self) {
        self.curr_line.clear();
        self.stdin.read_line(&mut self.curr_line);
        eprintln!("RUST got input: {}", self.curr_line);
    }

}
