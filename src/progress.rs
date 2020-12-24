use colored::Colorize;
extern crate term_size;
use std::io::{Write, stdout};
use std::process::exit;
pub struct Progress {
    pub current: usize,
    pub total: usize,
    text: String,
}

impl Progress {
    pub fn new(text: String) -> Self {
        Progress {
            current: 0,
            total: 0,
            text: text,
        }
    }

    pub fn print_frac(&self, frac: f32) {
        let mut label = (frac * 100_f32).round().to_string();
        label.push('%');

        let width = match term_size::dimensions() {
            Some(d) => d.0,
            None => custompanic!("Couldn't find terminal dimensions. This is a bug, report it!")
        };

        let prog = "=".repeat(width - label.chars().count() - &self.text.len() - 4);

        let label = format!("{} [{}] {}", &self.text, prog, label);
        print!("\r{}", label);
        stdout().flush();
    }
}
