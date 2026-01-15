#[macro_export]
macro_rules! log_println {
    () => (println!("[GAMEBOY]"));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("[GAMEBOY] ", $fmt), $($arg)*));
    ($fmt:expr) => (println!(concat!("[GAMEBOY] ", $fmt)));
}

#[macro_export]
macro_rules! log_print {
    () => (print!("[GAMEBOY]"));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!("[GAMEBOY] ", $fmt), $($arg)*));
    ($fmt:expr) => (print!(concat!("[GAMEBOY] ", $fmt)));
}
