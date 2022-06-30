#![allow(unused_macros)]

use chrono::Local;
use std::io;
#[macro_export]
macro_rules! debug {
    () => ({
        let line = line!();
        let file = file!();
        let _ = $crate::output("DEBUG",file,line,String::new());
    });
    ($($arg:tt)*) => ({
        let line = line!();
        let file = file!();
        let value = format!($($arg)*);
        let _ = $crate::output("DEBUG",file,line,value);
    })
}
pub fn output(level: &str, file: &str, line: u32, mut fmt: String) -> io::Result<()> {
    let file_line = if file.len() > 0 {
        format!("{}({}):", file, line)
    } else {
        String::new()
    };
    println!("{}", format!(
        "\x1b[32m[{}{}]{} {}\x1b[0m",
        Local::now().format("%Y-%m-%d %H:%M:%S%.3f ").to_string(),
        level,  file_line, fmt));
    return Ok(());
}

#[test]
fn ex() {
    debug!();
}