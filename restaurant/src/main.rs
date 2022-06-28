use restaurant::{self, front_of_house::hosting};
use std::{fmt::Result, io::Result as IoResult};
fn main() {
    println!("hello world!");
    hosting::add_to_wait_list();
    restaurant::eat_at_restaurant();
}
