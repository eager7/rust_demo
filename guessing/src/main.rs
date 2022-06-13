use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guessing program");

    let rand_num = rand::thread_rng().gen_range(1..101);
    println!("rand:{:?}", rand_num);

    loop {
        println!("input a number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("invalid input");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("input:{:?}", input);

        match rand_num.cmp(&input) {
            Ordering::Less => println!("smaller"),
            Ordering::Greater => println!("bigger"),
            Ordering::Equal => {
                println!("success!");
                break;
            }
        }
    }
}
