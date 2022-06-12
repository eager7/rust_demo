fn main() {
    println!("Hello, world!");
    _if();
    _match();
}

fn _if() {
    let x = 5;
    let y = if x == 5 { 10 } else { 15 };
    println!("x:{:?}, y:{:?}", x, y);
}

fn _match() {
    let day = 5;
    match day {
        0 | 6 => println!("weekend:"),
        b @ 1..=5 => println!("weekday:{:?}", b),
        c @ _ => println!("invalid:{:?}", c),
    }

    let mut day = day;
    match day {
        ref mut mr => println!("mut:{:?}", mr),
    }

    let pair = (0, -2);
    match pair {
        (0, y) => println!("x:{:?},y:{:?}", 0, y),
        (x, 0) => println!("x:{:?},y:{:?}", x, 0),
        _ => println!("x:{:?},y:{:?}", 0, 0),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x:{},y:{}", x, origin.y),
    }

    enum OptionInt {
        Value(i32),
        Missing,
    }
    let x = OptionInt::Value(5);
    match x {
        OptionInt::Value(i) if i > 5 => println!("bigger than 5"),
        OptionInt::Value(..) => println!("got an int"),
        OptionInt::Missing => println!("missing"),
    }
    match OptionInt::Missing {
        OptionInt::Value(i) if i > 5 => println!("bigger than 5"),
        OptionInt::Value(..) => println!("got an int"),
        OptionInt::Missing => println!("missing"),
    }

    let number = Some(7);
    let mut optional = Some(0);
    if let Some(i) = number {
        println!("matched:{:?}", i);
    } else {
        println!("did not match number");
    }
    while let Some(i) = optional {
        if i>9{
            println!("bigger than 9");
            optional = None;
        } else {
            println!("i:{:?}",i);
            optional = Some(i+1);
        }
    }
}
