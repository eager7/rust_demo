#[test]
fn vector() {
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
