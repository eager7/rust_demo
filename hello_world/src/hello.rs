// 测试函数只能用来测试，不能被调用
pub fn hello() {
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
