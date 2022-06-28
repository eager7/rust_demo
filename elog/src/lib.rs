// 日志打印颜色
const cRed: i32 = 91;
const cGreen: i32 = 92;
const cYellow: i32 = 93;
const cBlue: i32 = 94;
const cMagenta: i32 = 95;
const cBlueLight: i32 = 96;
const prefix: &str = "\x1b[";
const suffix: &str = "\x1b[0m";

pub fn debug() {
    println!("{}", format!("{}", 1));
}

#[cfg(test)]
mod tests {
    use crate::debug;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn d() {
        debug();
    }
}
