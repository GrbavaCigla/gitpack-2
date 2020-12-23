use colored::Colorize;
use std::process::exit;
// TODO: Logs

#[macro_export]
macro_rules! info {
    ($fmt:expr) => {
        println!("{} {}", "[:]".bold().blue(), $fmt.bold())
    };
    ($fmt:expr, $($args:tt)*) => {
        println!("{} {}", "[:]".bold().blue(), format!($fmt, $($args)+).bold())
    };
}

#[macro_export]
macro_rules! error {
    ($fmt:expr) => {
        eprintln!("{} {}", "[!]".bold().red(), $fmt.bold())
    };
    ($fmt:expr, $($args:tt)*) => {
        eprintln!("{} {}", "[!]".bold().red(), format!($fmt, $($args)+).bold())
    };
}

#[macro_export]
macro_rules! custompanic {
    ($fmt:expr) => {
        {
            eprintln!("{} {}", "[!]".bold().red(), $fmt.bold());
            exit(1);
        }
    };
    ($fmt:expr, $($args:tt)*) => {
        {
            eprintln!("{} {}", "[!]".bold().red(), format!($fmt, $($args)+).bold());
            exit(1);
        }
    };
}

pub trait GPError<T, E> {
    fn escape(self, err: &'static str) -> T;
}

impl<T, E> GPError<T, E> for Result<T, E> {
    fn escape(self, err: &'static str) -> T {
        match self {
            Ok(o) => o,
            Err(_) => custompanic!(err),
        }
    }
}