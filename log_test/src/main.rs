extern crate core;

fn main() {
    println!("log_test");
    wd_log_ex();
}

fn wd_log_ex() {
    //Set log level, default:Debug
    wd_log::set_level(wd_log::DEBUG);
    //Set the log output prefix, default:"wd_log"
    wd_log::set_prefix("T");
    //Whether to display the print time, default:true
    wd_log::show_time(true);
    //Whether to display the location, default:true
    wd_log::show_file_line(true);
    //Set output to a file, default:stdout
    // wd_log::output_to_file("./log.txt").expect("file open failed");

    wd_log::log_debug!("hello world\n");
    wd_log::log_debug_ln!("hello world");
    wd_log::log_info!("hello world\n");
    wd_log::log_info_ln!("hello world");
    wd_log::log_warn!("hello world\n");
    wd_log::log_warn_ln!("hello world");
    wd_log::log_error!("hello world\n");
    wd_log::log_error_ln!("hello world");
}
