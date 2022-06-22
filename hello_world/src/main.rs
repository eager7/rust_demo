use std::cell::Cell;
use std::fmt;

fn main() {
    _hello();
    _array();
    _vector();
    _string();
    _struct();
    _enum();
    _func();
    f3();
}

fn _hello() {
    println!("Hello, world!");

    let mut a1 = 5; // mut 表示可变变量，否则默认是常量，不允许重新赋值
    let a2: i32 = 5;
    assert_eq!(a1, a2);

    a1 = 6_6_6; // 下划线是间隔符，会被忽略

    println!("a1:{:?}", a1);
    let a1 = a1; // 重新变为常量
                 // a1 = 7; // cannot assign twice to immutable variable
    println!("a1:{:?}", a1);

    let b: u32 = 6;
    println!("b:{:?}", b);
    // assert_eq!(a1, b);// expected `i32`, found `u32`

    let c = 2.0f32;
    println!("c:{:?}", c);

    let (d, mut e) = (true, false);
    println!("{:?},{:?}", d, e);
    e = true;
    println!("{:?},{:?}", d, e);
}

fn _array() {
    let mut array: [i32; 3] = [0; 3];
    println!("{:?}", array); // [0, 0, 0]

    array[0] = 0;
    array[1] = 1;
    array[2] = 2;
    println!("{:?}", array); // [0, 1, 2]

    assert_eq!([1, 2], &array[1..]); // 截取从1到最后的部分数组

    for x in array {
        println!("{:?}", x);
    }
}

fn _vector() {
    let v: Vec<i32> = Vec::new();
    // v.push(1); // cannot borrow as mutable
    println!("{:?}", v); // []

    // 用宏创建向量

    let mut v: Vec<i32> = vec![];
    v.push(1);
    println!("{:?}", v); // [1]

    let mut v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v); // [1, 2, 3, 4, 5]
    println!("{:?}", v.pop()); // Some(5)
    println!("{:?}", Some(5));
    println!("{:?}", v); // [1, 2, 3, 4]

    v[0] = v[1] + v[2];
    println!("{:?}", v); // [5, 2, 3, 4]
}

fn _string() {
    let s: &str = "hello string";
    println!("{:?}", s);

    let str = s.to_string();
    println!("{:?}", str);

    let s: &'static str = "static hello";
    println!("{:?}", s);

    let mut s = String::new();
    println!("new:{:?}", s);
    s = String::from("string new ");
    println!("from:{:?}", s);
    s.push('w');
    s.push_str("orld");
    println!("push:{:?}", s);

    assert_eq!(s.pop(), Some('d'));
}

fn _struct() {
    // #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl fmt::Debug for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Point")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }
    let point = Point { x: 1, y: 2 };
    println!("point:{:?}", point);

    #[derive(Debug)]
    struct Color(u8, u8, u8);
    let android_green = Color(0xa4, 0xc6, 0x39);
    let Color(red, green, blue) = android_green;
    println!("color:{:?}", android_green);
    println!("red, green, blue,{:?},{:?},{:?}", red, green, blue);

    #[derive(Debug)]
    struct Digit(i32);
    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();
    println!("d:{:?}", d);

    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("integer_length:{:?}", integer_length);

    #[derive(Debug)]
    struct EmptyStruct;
    let empty = EmptyStruct;
    println!("empty:{:?}", empty);

    #[derive(Default, Debug)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point3d::default();
    let point = Point3d { y: 1, ..origin };
    let Point3d { x: x0, y: y0, .. } = point;
    println!("origin:{:?}", origin);
    println!("point:{:?}", point);
    println!("x:{:?},y:{:?},z:{:?}", x0, y0, point.z);

    #[derive(Debug)]
    struct MutPoint {
        x: i32,
        y: Cell<i32>,
    }
    let point = MutPoint {
        x: 5,
        y: Cell::new(6),
    };
    println!("mut point:{:?}", point);
    point.y.set(7);
    println!("mut point:{:?} {:?}", point.x, point.y);
}

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

fn _func() {
    assert_eq!(f1(1), f2(1));
}

fn f1(v: i32) -> i32 {
    return v + 1;
}
fn f2(v: i32) -> i32 {
    v + 1
}
fn f3() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        };
    };

    println!("The result is {}", result);
}
