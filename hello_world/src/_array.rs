#[test]
fn array() {
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
