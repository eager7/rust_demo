use colored::Colorize;

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

#[test]
fn color () {
    println!("{:?}", "this is blue".blue());
    "this is red".red();
    "this is red on blue".red().on_blue();
    "this is also red on blue".on_blue().red();
    "you can use truecolor values too!".truecolor(0, 255, 136);
    "background truecolor also works :)".on_truecolor(135, 28, 167);
    "you can also make bold comments".bold();
    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
    "or change advice. This is red".yellow().blue().red();
    "or clear things up. This is default color and style".red().bold().clear();
    "purple and magenta are the same".purple().magenta();
    "bright colors are also allowed".bright_blue().on_bright_white();
    "you can specify color by string".color("blue").on_color("red");
    "and so are normal and clear".normal().clear();
    String::from("this also works!").green().bold();
    format!("{:30}", "format works as expected. This will be padded".blue());
    format!("{:.3}", "and this will be green but truncated to 3 chars".green());
}

#[test]
fn wd_log_test(){
    //Set log level, default:Debug
    wd_log::set_level(wd_log::INFO);
    //Set the log output prefix, default:"wd_log"
    wd_log::set_prefix("WDLOG");
    //Whether to display the print time, default:true
    wd_log::show_time(false);
    //Whether to display the location, default:true
    wd_log::show_file_line(false);
    //Set output to a file, default:stdout
    // wd_log::output_to_file("./log.txt").expect("file open failed");

    wd_log::log_debug!("hello world");
    wd_log::log_debug_ln!("hello world");
    wd_log::log_info!("hello world");
    wd_log::log_info_ln!("hello world");
    wd_log::log_warn!("hello world");
    wd_log::log_warn_ln!("hello world");
    wd_log::log_error!("hello world");
    wd_log::log_error_ln!("hello world");
}