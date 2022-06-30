#![allow(unused_macros)]

use chrono::Local;
use std::io;

mod color;

#[macro_export]
macro_rules! logger {
    ($lvl:expr) => ({
        let line = line!();
        let file = file!();
        let _ = $crate::output($lvl,file,line,String::new());
    });
    ($lvl:expr, $($arg:tt)*) => ({
        let line = line!();
        let file = file!();
        let value = format!($($arg)*);
        let _ = $crate::output($lvl,file,line,value);
    })
}

#[macro_export]
macro_rules! trace {
    () => (logger!(color::Color::BrightGreen));
    ($($arg:tt)*) => (logger!(color::Color::BrightCyan,$($arg)*))
}

#[macro_export]
macro_rules! notice {
    () => (logger!(color::Color::BrightGreen));
    ($($arg:tt)*) => (logger!(color::Color::BrightGreen,$($arg)*))
}

#[macro_export]
macro_rules! debug {
    () => (logger!(color::Color::BrightBlue));
    ($($arg:tt)*) => (logger!(color::Color::BrightBlue,$($arg)*))
}

#[macro_export]
macro_rules! info {
    () => (logger!(color::Color::BrightYellow));
    ($($arg:tt)*) => (logger!(color::Color::BrightYellow,$($arg)*))
}

#[macro_export]
macro_rules! warn {
    () => (logger!(color::Color::BrightMagenta));
    ($($arg:tt)*) => (logger!(color::Color::BrightMagenta,$($arg)*))
}

#[macro_export]
macro_rules! error {
    () => (logger!(color::Color::Red));
    ($($arg:tt)*) => (logger!(color::Color::Red,$($arg)*))
}

pub fn output(level: color::Color, file: &str, line: u32, content: String) -> io::Result<()> {
    let file_line = format!("{}:{}  ", file, line);
    let buf = format!(
        "\x1b[{}m{}▶  {} {}\x1b[0m",
        level.to_fg_str().to_string(),
        Local::now().format("%Y-%m-%d %H:%M:%S%.3f ").to_string(),
        file_line,
        content
    );
    println!("{}", buf);
    return Ok(());
}

#[test]
fn ex() {
    logger!(color::Color::BrightCyan, "logger");
    trace!("trace");
    notice!("notice");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
    debug!("{}", 3);

    let c: color::Color = String::from("bright blue").into();
    println!("{}", c.to_fg_str())
}
