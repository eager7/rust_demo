#[test]
fn _enum() {
    #[derive(Debug)]
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }
    let x = Message::Move { x: 3, y: 4 };
    println!("x:{:?}", x);

    let x = Message::ChangeColor(1, 2, 3);
    println!("x:{:?}", x);

    let x = Message::Write(String::from("hello"));
    println!("x:{:?}", x);

    let x = Message::Quit;
    println!("x:{:?}", x);
}

#[test]
fn _enum2() {
    #[derive(PartialOrd, PartialEq)]
    pub enum Level {
        Error = 1,
        Warn,
        Info,
        Debug,
        Trace,
    }
    println!("{:?}", Level::Error < Level::Debug);
}
