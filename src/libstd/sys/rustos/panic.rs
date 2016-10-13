use super::terminal;

//#[lang = "panic_fmt"] #[inline(never)] #[cold]
pub extern fn panic_impl(msg: ::core::fmt::Arguments,
                         file: &'static str,
                         line: usize) -> !
{
  unsafe {
    use io::Write;
    let _ = write!(terminal::get_terminal(), "{}:{} {}", file, line, msg);
    ::core::intrinsics::abort();
  }
}
