use std::fmt;

#[test]
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
        y: std::cell::Cell<i32>,
    }
    let point = MutPoint {
        x: 5,
        y: std::cell::Cell::new(6),
    };
    println!("mut point:{:?}", point);
    point.y.set(7);
    println!("mut point:{:?} {:?}", point.x, point.y);
}
