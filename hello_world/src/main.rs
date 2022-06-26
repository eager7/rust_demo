use hello_world;
use std::cell::Cell;
use std::fmt;
use restaurant;

fn main() {
    println!("Hello, world!");
    hello_world::hello::hello();

    restaurant::eat_at_restaurant();
}
