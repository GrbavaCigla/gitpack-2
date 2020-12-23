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