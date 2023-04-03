macro_rules! kdbg {
    ($fmt:expr) => {
        println!(concat!("[DBG]  ", $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        println!(concat!("[DBG]  ", $fmt), $($arg)*);
    };
}

pub(crate) use kdbg;

macro_rules! kinfo {
    ($fmt:expr) => {
        println!(concat!("[INFO] ", $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        println!(concat!("[INFO] ", $fmt), $($arg)*);
    };
}

pub(crate) use kinfo;
