#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        print!("\x1b[90m");
        print!($($arg)*);
        println!("\x1b[0m");
    })
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        print!("\x1b[31m");
        print!($($arg)*);
        println!("\x1b[0m");
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        print!("\x1b[33m");
        print!($($arg)*);
        println!("\x1b[0m");
    };
}
