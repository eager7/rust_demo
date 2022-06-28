use hello_world;
use restaurant;
use std::cell::Cell;
use std::fmt;

fn main() {
    println!("Hello, world!");
    hello_world::hello::hello();

    restaurant::eat_at_restaurant();
}
