//#[macro_export]
macro_rules! __print(
  ($($arg:tt)*) => ({
    use io::Write;
    write!(::sys::terminal::get_terminal(), $($arg)*).ok();
  })
);

#[macro_export]
macro_rules! log(
  ($lvl: expr, $($arg:tt)*) => ({
    __print!("[{}:{} {}]: ", $lvl, file!(), line!());
    __print!($($arg)*);
    __print!("\n");
  })
);

#[macro_export]
macro_rules! debug(
  ($($arg:tt)*) => (log!("DEBUG", $($arg)*))
);

#[macro_export]
macro_rules! warn(
  ($($arg:tt)*) => (log!("WARN", $($arg)*))
);

#[macro_export]
macro_rules! info(
  ($($arg:tt)*) => (log!("INFO", $($arg)*))
);

#[macro_export]
macro_rules! trace(
  ($($arg:tt)*) => (log!("TRACE", $($arg)*))
);
