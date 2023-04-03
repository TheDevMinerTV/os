macro_rules! kinfo {
    ($fmt:expr) => {
        println!(concat!("[INFO] ", $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        println!(concat!("[INFO] ", $fmt), $($arg)*);
    };
}
