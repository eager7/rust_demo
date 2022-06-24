use restaurant::front_of_house::hosting;
use std::{fmt::Result,io::Result as IoResult};
fn main() {
    println!("hello world!");
    // crate::front_of_house::hosting::add_to_wait_list();
    // front_of_house::hosting::add_to_wait_list();
    hosting::eat_at_restaurant();
}