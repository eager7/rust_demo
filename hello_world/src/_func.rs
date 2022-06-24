#[test]
fn _func() {
    assert_eq!(f1(1), f2(1));
}

fn f1(v: i32) -> i32 {
    return v + 1;
}
fn f2(v: i32) -> i32 {
    v + 1
}
#[test]
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
