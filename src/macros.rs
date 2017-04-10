#[cfg(not(feature = "semihosting"))]
#[macro_export]
macro_rules! hprint {
    ($($arg:tt)*) => ();
}

#[cfg(not(feature = "semihosting"))]
#[macro_export]
macro_rules! hprintln {
    ($($arg:tt)*) => ();
}
