#[macro_export]
macro_rules! log {
    (ERROR, $($arg:tt)*) => {
        eprintln!("\x1b[31m[ERROR]\x1b[0m {}", format!($($arg)*)); // Red
    };
    (INFO, $($arg:tt)*) => {
        println!("\x1b[32m[INFO]\x1b[0m {}", format!($($arg)*)); // Green
    };
    (WARN, $($arg:tt)*) => {
        eprintln!("\x1b[33m[WARN]\x1b[0m {}", format!($($arg)*)); // Yellow
    };
    (CONTEXT, $($arg:tt)*) => {
        println!("\x1b[34m[CONTEXT]\x1b[0m {}", format!($($arg)*)); // Blue
    };
    (DEBUG, $($arg:tt)*) => {
        println!("\x1b[35m[DEBUG]\x1b[0m {}", format!($($arg)*)); // Magenta
    };
}
